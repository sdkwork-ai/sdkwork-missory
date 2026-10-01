//! Rule-based memory extraction from free text (PRD §17/§20/§21).
//!
//! Extraction is AI-assist only: every produced record is a `candidate` whose
//! origin is `fact` (stated in the source text) or `inference` (hedged wording),
//! and the user must confirm before it becomes an active fact.

use std::sync::Arc;

use sdkwork_missory_contract::dto::{
    Importance, Memory, MemoryOrigin, MemoryStatus, MemoryType, SourceKind,
};
use sdkwork_missory_spi::{MissoryScope, MissoryStore};
use time::OffsetDateTime;

/// Sentence separators used to split free text.
const SENTENCE_SEPARATORS: [char; 8] = ['。', '！', '？', '!', '?', ';', '；', '\n'];

/// Hedge words marking a sentence as an AI inference rather than a stated fact.
const HEDGES: [&str; 8] = [
    "可能", "也许", "大概", "好像", "听说", "maybe", "might", "seems",
];

/// Resolves the target person by scanning the text for known names and aliases.
pub async fn resolve_person_by_mention(
    text: &str,
    store: &Arc<dyn MissoryStore>,
    scope: &MissoryScope,
) -> Result<Option<u64>, sdkwork_missory_spi::MissoryStoreError> {
    let (persons, _) = store
        .list_persons(
            scope,
            sdkwork_missory_contract::ports::ListPersonsQuery {
                page: Some(1),
                page_size: Some(200),
                ..sdkwork_missory_contract::ports::ListPersonsQuery::default()
            },
        )
        .await?;
    let lowered = text.to_lowercase();
    for person in persons {
        if lowered.contains(&person.display_name.to_lowercase())
            || person
                .aliases
                .iter()
                .any(|alias| !alias.is_empty() && lowered.contains(&alias.to_lowercase()))
        {
            return Ok(Some(person.id));
        }
    }
    Ok(None)
}

/// Extracts candidate memories from a text without persisting them.
pub fn extract_candidates(person_id: u64, text: &str, now: OffsetDateTime) -> Vec<Memory> {
    let now_text = now
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
    let mut candidates = Vec::new();
    for raw_sentence in text.split(SENTENCE_SEPARATORS) {
        let sentence = raw_sentence.trim();
        if sentence.chars().count() < 4 {
            continue;
        }
        let Some((memory_type, title_prefix)) = classify(sentence) else {
            continue;
        };
        let hedged = HEDGES
            .iter()
            .any(|hedge| sentence.to_lowercase().contains(hedge));
        let content = sentence.to_string();
        if candidates
            .iter()
            .any(|memory: &Memory| memory.content == content)
        {
            continue;
        }
        let title: String = content.chars().take(20).collect();
        candidates.push(Memory {
            id: 0, // minted by the service before insert
            person_id,
            story_id: None,
            memory_type,
            title: Some(format!("{title_prefix}：{title}")),
            content,
            origin: if hedged {
                MemoryOrigin::Inference
            } else {
                MemoryOrigin::Fact
            },
            status: MemoryStatus::Candidate,
            confidence: if hedged { Some(0.6) } else { None },
            source_reason: if hedged {
                Some("表述带有不确定性词汇，标记为推断等待确认".to_string())
            } else {
                None
            },
            source_kind: SourceKind::AiExtraction,
            source_ref: None,
            importance: Some(Importance::Normal),
            occurred_at: Some(now_text.clone()),
            created_at: now_text.clone(),
            updated_at: now_text.clone(),
        });
    }
    candidates
}

/// Classifies one sentence into a memory type and a Chinese title prefix.
fn classify(sentence: &str) -> Option<(MemoryType, &'static str)> {
    let lowered = sentence.to_lowercase();
    if contains_any(&lowered, &["答应", "承诺", "答应帮", "promise"]) {
        return Some((MemoryType::Commitment, "承诺"));
    }
    if contains_any(&lowered, &["喜欢", "爱吃", "爱好", " likes ", "likes "]) {
        return Some((MemoryType::Preference, "偏好"));
    }
    if contains_any(
        &lowered,
        &[
            "离职",
            "跳槽",
            "换工作",
            "创业",
            "搬家",
            "结婚",
            "毕业",
            "准备",
            "正在",
        ],
    ) {
        return Some((MemoryType::Temporal, "近况"));
    }
    if contains_any(
        &lowered,
        &["去过", "去了", "一起", "旅行", "见过", "聊了", "聊到"],
    ) {
        return Some((MemoryType::Episodic, "经历"));
    }
    if contains_any(
        &lowered,
        &["同学", "同事", "朋友", "客户", "合作伙伴", "老师"],
    ) {
        return Some((MemoryType::Relationship, "关系"));
    }
    if contains_any(&lowered, &["是", "在做", "在从事", " works ", "is a"]) {
        return Some((MemoryType::Semantic, "事实"));
    }
    None
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_stated_preference_sentence_when_extracting_then_fact_candidate_is_created() {
        let candidates = extract_candidates(
            1,
            "李明喜欢摄影。他最近在准备创业。",
            OffsetDateTime::UNIX_EPOCH,
        );
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].memory_type, MemoryType::Preference);
        assert_eq!(candidates[0].origin, MemoryOrigin::Fact);
        assert_eq!(candidates[0].status, MemoryStatus::Candidate);
    }

    #[test]
    fn given_hedged_sentence_when_extracting_then_inference_metadata_is_attached() {
        let candidates = extract_candidates(1, "听说李明可能要创业", OffsetDateTime::UNIX_EPOCH);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].origin, MemoryOrigin::Inference);
        assert_eq!(candidates[0].confidence, Some(0.6));
        assert!(candidates[0].source_reason.is_some());
    }

    #[test]
    fn given_commitment_sentence_when_extracting_then_commitment_type_is_assigned() {
        let candidates =
            extract_candidates(1, "我答应帮李明介绍一个客户", OffsetDateTime::UNIX_EPOCH);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].memory_type, MemoryType::Commitment);
    }

    #[test]
    fn given_short_noise_when_extracting_then_nothing_is_created() {
        let candidates = extract_candidates(1, "哈哈 嗯 好的", OffsetDateTime::UNIX_EPOCH);
        assert!(candidates.is_empty());
    }
}
