//! The rule-based social agent: Q&A, briefings, message drafts, chat summaries.
//!
//! Deterministic composition from structured context only — no fabrication
//! (PRD §33 AI safety rules). When a `SocialTextModel` is configured, drafts and
//! summaries may be refined through it with an automatic fallback to the
//! deterministic text on any model error.

use std::sync::Arc;

use sdkwork_missory_contract::dto::{
    AssistantAnswer, AssistantScenario, Briefing, Citation, CitationKind, Memory, MemoryStatus,
    MemoryType, MessageDraft, MessageDraftRequest, MessageTone, Person, PersonDetail,
};
use sdkwork_missory_spi::{MissoryStoreError, SocialTextModel, SocialTextPrompt};
use time::OffsetDateTime;

/// Builds a citation with the wire id form.
fn citation(kind: CitationKind, id: u64) -> Citation {
    Citation {
        kind,
        id: id.to_string(),
    }
}

fn person_mentioned(person: &Person, question: &str) -> bool {
    let lowered = question.to_lowercase();
    lowered.contains(&person.display_name.to_lowercase())
        || person
            .aliases
            .iter()
            .any(|alias| !alias.is_empty() && lowered.contains(&alias.to_lowercase()))
}

fn parse_instant(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339).ok()
}

/// Answers a natural-language question deterministically.
/// Answers "谁喜欢X？" from confirmed preference memories (memory-driven Q&A).
fn memory_driven_preference_answer(
    memory_term: &str,
    persons: &[Person],
    memories: &[Memory],
) -> Option<AssistantAnswer> {
    if memory_term.is_empty() {
        return None;
    }
    let matched: Vec<&Person> = persons
        .iter()
        .filter(|person| {
            memories.iter().any(|memory| {
                memory.person_id == person.id
                    && memory.memory_type == MemoryType::Preference
                    && memory.status == MemoryStatus::Confirmed
                    && memory.content.contains(memory_term)
            })
        })
        .collect();
    if matched.is_empty() {
        return None;
    }
    let names = matched
        .iter()
        .map(|person| person.display_name.as_str())
        .collect::<Vec<_>>()
        .join("、");
    let mut citations = matched
        .iter()
        .map(|person| citation(CitationKind::Person, person.id))
        .collect::<Vec<_>>();
    citations.extend(
        memories
            .iter()
            .filter(|memory| {
                memory.memory_type == MemoryType::Preference
                    && memory.status == MemoryStatus::Confirmed
                    && memory.content.contains(memory_term)
                    && matched.iter().any(|person| person.id == memory.person_id)
            })
            .map(|memory| citation(CitationKind::Memory, memory.id)),
    );
    Some(AssistantAnswer {
        answer: format!("喜欢{memory_term}的人有：{names}。"),
        citations,
    })
}

/// Extracts the preference keyword after 喜欢 (谁喜欢跑步？ -> 跑步); empty when
/// the question carries no 喜欢 phrase.
fn extract_preference_term(lowered: &str) -> String {
    let Some(index) = lowered.find("喜欢") else {
        return String::new();
    };
    let after = &lowered[index + "喜欢".len()..];
    let end = after
        .find([
            '？', '?', '，', ',', '。', '、', '；', ';', '！', '!', '吗', '呢', '吧',
        ])
        .unwrap_or(after.len());
    after[..end].trim().to_owned()
}

/// Answers a natural-language question deterministically over the owner's
/// people and confirmed memories (no fabrication: every claim cites records).
pub fn answer_question(question: &str, persons: &[Person], memories: &[Memory]) -> AssistantAnswer {
    let lowered = question.to_lowercase();

    // Person-specific intents take precedence when a known person is mentioned.
    if let Some(person) = persons
        .iter()
        .find(|person| person_mentioned(person, question))
    {
        let person_memories: Vec<&Memory> = memories
            .iter()
            .filter(|memory| memory.person_id == person.id)
            .collect();
        if lowered.contains("多久没") || (lowered.contains("多久") && lowered.contains("联系"))
        {
            return days_since_contact_answer(person, &person_memories);
        }
        if lowered.contains("什么关系") || lowered.contains("的关系") {
            return relationship_answer(person);
        }
        if lowered.contains("什么时候认识") || lowered.contains("认识多久") {
            return known_since_answer(person);
        }
        if lowered.contains("上次") || lowered.contains("聊") {
            return last_conversation_answer(person, &person_memories);
        }
        return person_overview_answer(person, &person_memories);
    }

    let memory_term = extract_preference_term(&lowered);
    if let Some(answer) = memory_driven_preference_answer(&memory_term, persons, memories) {
        return answer;
    }

    // Interest-style people search: 谁喜欢摄影 / 哪些朋友做 AI.
    let keyword = persons
        .iter()
        .flat_map(|person| {
            person
                .interests
                .iter()
                .chain(person.tags.iter())
                .chain(person.preferences.iter())
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|value| !value.is_empty())
        .find(|value| lowered.contains(&value.to_lowercase()));
    if let Some(interest) = keyword {
        let matched: Vec<&Person> = persons
            .iter()
            .filter(|person| {
                person
                    .interests
                    .iter()
                    .chain(person.tags.iter())
                    .chain(person.preferences.iter())
                    .any(|value| value.eq_ignore_ascii_case(&interest))
            })
            .collect();
        if !matched.is_empty() {
            let names = matched
                .iter()
                .map(|person| person.display_name.as_str())
                .collect::<Vec<_>>()
                .join("、");
            return AssistantAnswer {
                answer: format!("{interest}相关的联系人有：{names}。"),
                citations: matched
                    .iter()
                    .map(|person| citation(CitationKind::Person, person.id))
                    .collect(),
            };
        }
    }

    // Fallback: explain the answerable surface instead of guessing (PRD §33).
    AssistantAnswer {
        answer: "没有找到匹配的人物。可以试试：\"李明是谁？\"、\
                 \"我和李明是什么关系？\"、\"我多久没联系李明了？\" 或 \"谁喜欢摄影？\"。"
            .to_string(),
        citations: vec![],
    }
}

fn days_since_contact_answer(person: &Person, memories: &[&Memory]) -> AssistantAnswer {
    let last = person
        .last_contacted_at
        .as_deref()
        .and_then(parse_instant)
        .or_else(|| {
            memories
                .iter()
                .filter_map(|memory| parse_instant(&memory.created_at))
                .max()
        });
    match last {
        Some(instant) => {
            let days = (OffsetDateTime::now_utc() - instant).whole_days().max(0);
            AssistantAnswer {
                answer: format!(
                    "您和{}大约 {} 天没有联系了。最近的一条记忆：{}。",
                    person.display_name,
                    days,
                    memories
                        .first()
                        .map_or("暂无记录", |memory| memory.content.as_str())
                ),
                citations: vec![citation(CitationKind::Person, person.id)],
            }
        }
        None => AssistantAnswer {
            answer: format!(
                "还没有{}的最近联系记录，可以先创建一条记忆。",
                person.display_name
            ),
            citations: vec![citation(CitationKind::Person, person.id)],
        },
    }
}

fn relationship_answer(person: &Person) -> AssistantAnswer {
    let mut answer = format!("{} 是您的重要联系人。", person.display_name);
    if let Some(notes) = person.notes.as_deref() {
        let _ = std::fmt::Write::write_fmt(&mut answer, format_args!("备注：{notes}。"));
    }
    if !person.tags.is_empty() {
        let _ = std::fmt::Write::write_fmt(
            &mut answer,
            format_args!("标签：{}。", person.tags.join("、")),
        );
    }
    AssistantAnswer {
        answer,
        citations: vec![citation(CitationKind::Person, person.id)],
    }
}

fn known_since_answer(person: &Person) -> AssistantAnswer {
    match parse_instant(&person.created_at) {
        Some(instant) => {
            let years = (OffsetDateTime::now_utc().year() - instant.year()).max(0);
            AssistantAnswer {
                answer: format!(
                    "档案里{}最早记录于 {} 年，至今约 {years} 年。",
                    person.display_name,
                    instant.year()
                ),
                citations: vec![citation(CitationKind::Person, person.id)],
            }
        }
        None => AssistantAnswer {
            answer: format!("暂时没有{}的认识时间记录。", person.display_name),
            citations: vec![citation(CitationKind::Person, person.id)],
        },
    }
}

fn last_conversation_answer(person: &Person, memories: &[&Memory]) -> AssistantAnswer {
    match memories.first() {
        Some(memory) => AssistantAnswer {
            answer: format!(
                "和{}最近的一条记录（{}）：{}",
                person.display_name, memory.created_at, memory.content
            ),
            citations: vec![
                citation(CitationKind::Person, person.id),
                citation(CitationKind::Memory, memory.id),
            ],
        },
        None => AssistantAnswer {
            answer: format!("还没有和{}的对话或记忆记录。", person.display_name),
            citations: vec![citation(CitationKind::Person, person.id)],
        },
    }
}

fn person_overview_answer(person: &Person, memories: &[&Memory]) -> AssistantAnswer {
    let mut parts = vec![person.display_name.clone()];
    if let Some(title) = person.title.as_deref() {
        parts.push(title.to_string());
    }
    if let Some(company) = person.company.as_deref() {
        parts.push(format!("任职于{company}"));
    }
    if !person.interests.is_empty() {
        parts.push(format!("兴趣：{}", person.interests.join("、")));
    }
    let mut answer = format!("{}。", parts.join("，"));
    let confirmed: Vec<&Memory> = memories
        .iter()
        .copied()
        .filter(|memory| memory.status == MemoryStatus::Confirmed)
        .take(3)
        .collect();
    if !confirmed.is_empty() {
        answer.push_str("最近确认的记忆：");
        answer.push_str(
            &confirmed
                .iter()
                .map(|memory| memory.content.as_str())
                .collect::<Vec<_>>()
                .join("；"),
        );
        answer.push('。');
    }
    let mut citations = vec![citation(CitationKind::Person, person.id)];
    citations.extend(
        confirmed
            .iter()
            .map(|memory| citation(CitationKind::Memory, memory.id)),
    );
    AssistantAnswer { answer, citations }
}

/// Builds the deterministic meeting briefing, optionally refined by the model.
pub async fn compose_briefing(
    model: &Option<Arc<dyn SocialTextModel>>,
    detail: &PersonDetail,
    now: String,
) -> Briefing {
    let person = &detail.person;
    let mut topics: Vec<String> = person.interests.iter().take(3).cloned().collect();
    for memory in &detail.recent_memories {
        topics.push(memory.content.chars().take(18).collect());
        if topics.len() >= 5 {
            break;
        }
    }
    let known_days = detail.stats.known_days;
    let days_silent = detail.stats.days_since_last_contact.map_or_else(
        || "还没有联系记录".to_string(),
        |days| format!("距离上次联系已经 {days} 天"),
    );
    let commitments = detail.commitments.first().map_or_else(
        || "没有待履行的承诺".to_string(),
        |memory| format!("你答应过 TA：{}", memory.content),
    );
    let mut briefing = format!(
        "【见面简报 · {now}】\n认识约 {known_days} 天；{days_silent}。\n{commitments}\n建议话题：{}。",
        if topics.is_empty() {
            "先聊聊近况".to_string()
        } else {
            topics.join("、")
        }
    );
    if let Some(model) = model {
        briefing = refine(
            model,
            "为一次见面生成简短中文简报，保持事实不变",
            &briefing,
            briefing.clone(),
        )
        .await;
    }
    Briefing {
        person_id: person.id,
        briefing,
        topics,
    }
}

/// Composes a draft message (never auto-sent), optionally refined by the model.
pub async fn compose_message_draft(
    model: &Option<Arc<dyn SocialTextModel>>,
    detail: &PersonDetail,
    request: &MessageDraftRequest,
) -> MessageDraft {
    let person = &detail.person;
    let tone = MessageTone::or_natural(request.tone);
    let interest = person.interests.first().cloned().unwrap_or_default();
    let recent = detail
        .recent_memories
        .first()
        .map(|memory| memory.content.chars().take(20).collect::<String>())
        .unwrap_or_default();
    let topic_hint = if recent.is_empty() {
        "上次聊的事"
    } else {
        recent.as_str()
    };
    let body = match request.scenario {
        AssistantScenario::Birthday => match tone {
            MessageTone::Brief => format!("{},生日快乐！祝你新的一岁顺利。", person.display_name),
            MessageTone::Formal => format!(
                "{}，祝您生日快乐，工作顺利，身体健康。",
                person.display_name
            ),
            MessageTone::Humorous => format!(
                "{}，又长大一岁啦！生日快乐，蛋糕要吃最大块的。",
                person.display_name
            ),
            _ => format!(
                "{},生日快乐！愿你在{}的路上一直开心，新的一岁万事顺意。",
                person.display_name,
                if interest.is_empty() {
                    "喜欢的事情"
                } else {
                    interest.as_str()
                }
            ),
        },
        AssistantScenario::CheckIn => match tone {
            MessageTone::Brief => format!("{},好久不见，最近怎么样？", person.display_name),
            _ => format!(
                "{},好久没联系了。还记得{topic_hint}吗？最近怎么样，有空聊聊？",
                person.display_name
            ),
        },
        AssistantScenario::Congratulations => {
            format!("{},听说{topic_hint}，恭喜恭喜！", person.display_name)
        }
        AssistantScenario::ThankYou => format!(
            "{},谢谢你上次帮忙，一直记在心里，多谢！",
            person.display_name
        ),
        AssistantScenario::Custom => request
            .note
            .clone()
            .unwrap_or_else(|| format!("{},想跟你聊几句。", person.display_name)),
    };
    let mut draft = body;
    if let Some(model) = model {
        draft = refine(
            model,
            "按给定语气润色这条中文消息草稿，保持事实不变",
            &format!("语气：{tone:?}；原文：{draft}"),
            draft,
        )
        .await;
    }
    MessageDraft {
        person_id: person.id,
        draft,
        tone,
        disclaimer: "草稿仅供参考，Missory 不会自动发送任何消息。".to_string(),
    }
}

/// Builds the deterministic story summary, optionally refined by the model.
pub async fn compose_story_summary(
    model: &Option<Arc<dyn SocialTextModel>>,
    title: &str,
    participant_names: &[String],
    location: Option<&str>,
    started_at: Option<&str>,
    memories: &[Memory],
) -> String {
    let who = if participant_names.is_empty() {
        "我".to_string()
    } else {
        format!("我和{}", participant_names.join("、"))
    };
    let when = started_at.unwrap_or("不久前");
    let where_label = location.unwrap_or("路上");
    let mut summary = format!(
        "《{title}》：{who}于{when}在{where_label}的共同经历，共记录 {} 条记忆。",
        memories.len()
    );
    if !memories.is_empty() {
        summary.push_str("包括：");
        summary.push_str(
            &memories
                .iter()
                .map(|memory| memory.content.as_str())
                .take(5)
                .collect::<Vec<_>>()
                .join("；"),
        );
        summary.push('。');
    }
    if let Some(model) = model {
        summary = refine(
            model,
            "把这段经历改写成一段流畅的中文回忆",
            &summary,
            summary.clone(),
        )
        .await;
    }
    summary
}

/// Builds the chat summary line shown above extracted candidates.
pub fn compose_chat_summary(person_name: &str, candidates: &[Memory]) -> String {
    if candidates.is_empty() {
        return format!("与{person_name}的对话没有提取到可保存的记忆。");
    }
    let commitments = candidates
        .iter()
        .filter(|memory| memory.memory_type == MemoryType::Commitment)
        .count();
    let mut summary = format!(
        "与{person_name}的对话共提取 {} 条候选记忆",
        candidates.len()
    );
    if commitments > 0 {
        let _ =
            std::fmt::Write::write_fmt(&mut summary, format_args!("（其中承诺 {commitments} 条）"));
    }
    summary.push_str("，确认后才会成为正式记忆。");
    summary
}

async fn refine(
    model: &Arc<dyn SocialTextModel>,
    instruction: &str,
    context: &str,
    fallback: String,
) -> String {
    let prompt = SocialTextPrompt {
        instruction: instruction.to_string(),
        context: context.to_string(),
    };
    match model.complete(&prompt).await {
        Ok(text) if !text.trim().is_empty() => text,
        Ok(_) => fallback,
        Err(error) => {
            if let MissoryStoreError::Storage(detail) = error {
                tracing::warn!(
                    detail = %detail,
                    "social text model failed; using deterministic text"
                );
            }
            fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn person(id: u64, name: &str) -> Person {
        Person {
            id,
            display_name: name.to_string(),
            aliases: vec![],
            gender: None,
            birthday: None,
            city: None,
            title: Some("产品经理".to_string()),
            company: None,
            avatar_url: None,
            tags: vec![],
            interests: vec!["摄影".to_string()],
            preferences: vec![],
            bio: None,
            contact_channels: BTreeMap::default(),
            notes: None,
            last_contacted_at: None,
            created_at: "2014-09-01T00:00:00Z".to_string(),
            updated_at: "2026-10-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn given_who_is_question_when_answering_then_person_overview_is_returned() {
        let answer = answer_question("李明是谁？", &[person(1, "李明")], &[]);
        assert!(answer.answer.contains("李明"));
        assert_eq!(
            answer.citations.first().map(|item| item.kind),
            Some(CitationKind::Person)
        );
    }

    #[test]
    fn given_interest_question_when_answering_then_matching_person_is_listed() {
        let answer = answer_question("谁喜欢摄影？", &[person(2, "王芳")], &[]);
        assert!(answer.answer.contains("王芳"));
    }

    #[test]
    fn given_confirmed_preference_memory_when_answering_then_memory_driven_match_is_returned() {
        let mut person = person(3, "王芳");
        person.interests = vec![]; // profile has no interests; the memory does
        let memory = Memory {
            id: 30,
            person_id: 3,
            story_id: None,
            memory_type: MemoryType::Preference,
            title: None,
            content: "王芳喜欢跑步".to_string(),
            origin: sdkwork_missory_contract::dto::MemoryOrigin::Fact,
            status: MemoryStatus::Confirmed,
            confidence: None,
            source_reason: None,
            source_kind: sdkwork_missory_contract::dto::SourceKind::UserInput,
            source_ref: None,
            importance: None,
            occurred_at: None,
            created_at: "2026-10-01T00:00:00Z".to_string(),
            updated_at: "2026-10-01T00:00:00Z".to_string(),
        };
        let answer = answer_question("谁喜欢跑步？", &[person], &[memory]);
        assert!(answer.answer.contains("王芳"), "answer: {}", answer.answer);
        assert!(answer
            .citations
            .iter()
            .any(|c| c.kind == CitationKind::Memory));
    }

    #[test]
    fn given_unknown_person_when_answering_then_guidance_is_returned_without_fabrication() {
        let answer = answer_question("张三是谁？", &[person(1, "李明")], &[]);
        assert!(answer.answer.contains("没有找到"));
        assert!(answer.citations.is_empty());
    }
}
