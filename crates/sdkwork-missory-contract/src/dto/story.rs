//! Story DTOs (PRD §22).

use sdkwork_utils_rust::serde_uint64;
use serde::{Deserialize, Serialize};

/// A story bundling shared experiences into one narrative (PRD §22).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    /// Snowflake story id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub id: u64,
    /// Story title.
    pub title: String,
    /// AI or user summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Origin of the summary (fact = user-written, inference = AI draft).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_origin: Option<super::MemoryOrigin>,
    /// Participating person ids (wire: strings).
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::wire::serialize_u64_vec",
        deserialize_with = "crate::wire::deserialize_u64_vec"
    )]
    pub participant_ids: Vec<u64>,
    /// Linked memory ids (wire: strings).
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::wire::serialize_u64_vec",
        deserialize_with = "crate::wire::deserialize_u64_vec"
    )]
    pub memory_ids: Vec<u64>,
    /// Story start (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Story end (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Location label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Create/update request for a story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryUpsertRequest {
    /// Story title (1..=200 chars).
    pub title: String,
    /// Participating person ids (wire: strings).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::wire::deserialize_u64_vec_option"
    )]
    pub participant_ids: Option<Vec<u64>>,
    /// Linked memory ids (wire: strings).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::wire::deserialize_u64_vec_option"
    )]
    pub memory_ids: Option<Vec<u64>>,
    /// Story start (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Story end (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Location label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
