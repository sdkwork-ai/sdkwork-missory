//! Assistant, home digest, and briefing DTOs (PRD §23–§26, draft-only).

use sdkwork_utils_rust::serde_uint64;
use serde::{Deserialize, Serialize};

use super::Memory;

/// Free-form question for the social agent (PRD §23).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantQueryRequest {
    /// The user question (1..=2000 chars).
    pub question: String,
}

/// What kind of record a citation points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationKind {
    /// A person record.
    Person,
    /// A memory record.
    Memory,
    /// A relationship edge.
    Relationship,
    /// A story.
    Story,
}

/// One source record backing an assistant answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    /// Citation target kind.
    pub kind: CitationKind,
    /// Referenced record id (wire: string form of the u64 id).
    pub id: String,
}

/// Assistant answer with citations (no fabrication: citations list every used record).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantAnswer {
    /// Answer text.
    pub answer: String,
    /// Records used to compose the answer.
    pub citations: Vec<Citation>,
}

/// Meeting-briefing request (PRD §54: 明天见他聊什么).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BriefingRequest {
    /// Person to brief about (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
}

/// Meeting briefing built from profile, relationship, memories, and commitments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Briefing {
    /// Briefed person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// Briefing text.
    pub briefing: String,
    /// Suggested topics.
    pub topics: Vec<String>,
}

/// Message draft scenario (PRD §26).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssistantScenario {
    /// Birthday greeting.
    Birthday,
    /// Long-time-no-talk check-in.
    CheckIn,
    /// Congratulations.
    Congratulations,
    /// Thank-you note.
    ThankYou,
    /// Custom message.
    Custom,
}

/// Message draft tone (PRD §26).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageTone {
    /// Short and to the point.
    Brief,
    /// Natural everyday voice.
    Natural,
    /// Warm and caring.
    Warm,
    /// Light-hearted.
    Humorous,
    /// Formal.
    Formal,
    /// Friendly buddy voice.
    Friendly,
}

impl MessageTone {
    /// Normalized tone with `natural` as the default.
    pub fn or_natural(value: Option<Self>) -> Self {
        value.unwrap_or(Self::Natural)
    }
}

/// Message draft request (PRD §26).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDraftRequest {
    /// Recipient person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// Draft scenario.
    pub scenario: AssistantScenario,
    /// Draft tone; defaults to `natural`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tone: Option<MessageTone>,
    /// Extra user note folded into the draft.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// A generated message draft. Draft-only: the product never sends it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDraft {
    /// Recipient person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// Draft text.
    pub draft: String,
    /// Tone used.
    pub tone: MessageTone,
    /// Fixed disclaimer reminding the user that nothing is auto-sent.
    pub disclaimer: String,
}

/// Chat-text summary request (PRD §21).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSummaryRequest {
    /// Chat owner person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// Pasted (authorized) chat text (1..=20000 chars).
    pub text: String,
}

/// Chat summary plus candidate memories extracted from the text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSummary {
    /// Chat owner person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// Summary text.
    pub summary: String,
    /// Candidate memories (status `candidate`), awaiting confirmation.
    pub candidate_memories: Vec<Memory>,
}

/// Home page digest (PRD §10: 今天，有谁值得你想起？).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeDigest {
    /// Today's reminders, most urgent first.
    pub today_reminders: Vec<super::Reminder>,
    /// Recent memories, newest first.
    pub recent_memories: Vec<Memory>,
    /// Recently touched persons, newest first.
    pub recent_persons: Vec<super::Person>,
}
