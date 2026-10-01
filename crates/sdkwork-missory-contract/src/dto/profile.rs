//! Owner (my) profile DTOs (PRD §11).

use sdkwork_utils_rust::serde_uint64;
use serde::{Deserialize, Serialize};

/// The owner's own profile (PRD §11).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyProfile {
    /// Owner user id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub user_id: u64,
    /// Display name.
    pub display_name: String,
    /// Nickname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Occupation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    /// Company.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Education history summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,
    /// Interests.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interests: Vec<String>,
    /// Things the owner likes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub likes: Vec<String>,
    /// Things the owner dislikes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dislikes: Vec<String>,
    /// Preferred communication style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_style: Option<String>,
    /// Free-form bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Create/update request for the owner profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyProfileUpsertRequest {
    /// Display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Nickname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Occupation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    /// Company.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Education history summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,
    /// Interests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>,
    /// Things the owner likes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<String>>,
    /// Things the owner dislikes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dislikes: Option<Vec<String>>,
    /// Preferred communication style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_style: Option<String>,
    /// Free-form bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}
