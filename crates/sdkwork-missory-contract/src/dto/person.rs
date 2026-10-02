//! Person, relationship, and timeline DTOs.

use sdkwork_utils_rust::serde_uint64;
use serde::{Deserialize, Serialize};

/// Relationship type between the owner and a person (PRD §14.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    /// Family member.
    Family,
    /// Friend.
    Friend,
    /// Classmate.
    Classmate,
    /// Colleague.
    Colleague,
    /// Client.
    Client,
    /// Business partner.
    Partner,
    /// Teacher.
    Teacher,
    /// Student.
    Student,
    /// Neighbor.
    Neighbor,
    /// Spouse or romantic partner.
    Spouse,
    /// Anything else.
    Other,
}

impl RelationshipType {
    /// All relationship types, in canonical order.
    pub fn all() -> &'static [Self] {
        [
            Self::Family,
            Self::Friend,
            Self::Classmate,
            Self::Colleague,
            Self::Client,
            Self::Partner,
            Self::Teacher,
            Self::Student,
            Self::Neighbor,
            Self::Spouse,
            Self::Other,
        ]
        .as_slice()
    }
}

/// Owner-assigned importance of a person or memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    /// Low importance.
    Low,
    /// Normal importance (default).
    Normal,
    /// High importance.
    High,
    /// Core relationship or memory.
    Core,
}

impl Importance {
    /// Normalized value with `normal` as the default.
    pub fn or_normal(value: Option<Self>) -> Self {
        value.unwrap_or(Self::Normal)
    }
}

/// A person in the owner's social memory (PRD §12).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    /// Snowflake person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub id: u64,
    /// Display name.
    pub display_name: String,
    /// Alternate names or nicknames.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    /// Gender label, free-form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Birthday as `MM-DD` or `YYYY-MM-DD`; drives birthday reminders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Job title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Company.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Avatar URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Free-form tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Interests (for example photography, football).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interests: Vec<String>,
    /// Preferences (likes, dislikes, favorite things).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferences: Vec<String>,
    /// Free-form bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Contact channels keyed by kind (for example `wechat`, `phone`).
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub contact_channels: std::collections::BTreeMap<String, String>,
    /// Owner notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Last contact timestamp (RFC 3339), if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_contacted_at: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Create/update request for a person.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonUpsertRequest {
    /// Display name (required, 1..=120 chars).
    pub display_name: String,
    /// Alternate names or nicknames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Gender label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Birthday as `MM-DD` or `YYYY-MM-DD`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Job title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Company.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Avatar URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Free-form tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Interests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>,
    /// Preferences.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Vec<String>>,
    /// Free-form bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Contact channels keyed by kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_channels: Option<std::collections::BTreeMap<String, String>>,
    /// Owner notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Optional initial relationship types (create only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_types: Option<Vec<RelationshipType>>,
}

/// Owner-to-person relationship edge (PRD §14).
///
/// One edge per (owner, person) carrying multiple relationship types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    /// Snowflake relationship id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub id: u64,
    /// Related person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// One or more relationship types (multiple allowed).
    pub relationship_types: Vec<RelationshipType>,
    /// When the owner met the person (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Last contact timestamp (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_contacted_at: Option<String>,
    /// Relationship description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Importance of the relationship.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<Importance>,
    /// Custom long-uncontacted cycle in days; `None` uses the default cycle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_cycle_days: Option<i64>,
    /// Owner notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Create/update request for the owner-to-person relationship edge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipUpsertRequest {
    /// One or more relationship types (required).
    pub relationship_types: Vec<RelationshipType>,
    /// When the owner met the person (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Last contact timestamp (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_contacted_at: Option<String>,
    /// Relationship description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Importance of the relationship.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<Importance>,
    /// Custom long-uncontacted cycle in days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_cycle_days: Option<i64>,
    /// Owner notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Aggregated person profile view (PRD §13).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDetail {
    /// The person resource.
    pub person: Person,
    /// Relationship edges (owner → person); empty when none was established.
    pub relationships: Vec<Relationship>,
    /// Most recent memories, newest first.
    pub recent_memories: Vec<crate::dto::Memory>,
    /// Open commitments (promise memories), newest first.
    pub commitments: Vec<crate::dto::Memory>,
    /// Stories the person participates in.
    pub stories: Vec<crate::dto::Story>,
    /// Computed stats.
    pub stats: PersonDetailStats,
}

/// Computed stats for a person detail view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDetailStats {
    /// Number of memories linked to the person.
    pub memory_count: i64,
    /// Number of stories including the person.
    pub story_count: i64,
    /// Days since the relationship started (falls back to person creation).
    pub known_days: i64,
    /// Days since last contact, when a last-contact timestamp exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_last_contact: Option<i64>,
}

/// Timeline entry kind (PRD §16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineEntryKind {
    /// Relationship start (when we met).
    RelationshipStart,
    /// Memory event.
    Memory,
    /// Story.
    Story,
    /// Contact touchpoint.
    Contact,
}

/// One entry on a person's relationship timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEntry {
    /// When the entry happened (RFC 3339).
    pub occurred_at: String,
    /// Entry kind.
    pub kind: TimelineEntryKind,
    /// Human title.
    pub title: String,
    /// Optional detail text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Linked memory id, when the entry is a memory (wire: string).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_uint64::option::serialize",
        deserialize_with = "serde_uint64::option::deserialize"
    )]
    pub memory_id: Option<u64>,
    /// Linked story id, when the entry is a story (wire: string).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_uint64::option::serialize",
        deserialize_with = "serde_uint64::option::deserialize"
    )]
    pub story_id: Option<u64>,
}
