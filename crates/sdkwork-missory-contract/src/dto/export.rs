//! Whole-account data export DTOs (PRD §9 privacy: data deletion and export).

use serde::{Deserialize, Serialize};

use super::{Memory, MyProfile, Person, Relationship, Reminder, Story};

/// Create request for a whole-account data export document.
///
/// The P0 export is whole-account by design (privacy requirement); the empty
/// body keeps the create-pattern contract shape and leaves room for future
/// section filters without a breaking change.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataExportCreateRequest {}

/// One whole-account export document: the owner's profile plus every owned
/// domain collection under the request context scope (tenant + user).
///
/// Collections are bounded by personal-scale design; ids keep the int64-as-
/// string wire rule via the referenced item DTOs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataExport {
    /// Export timestamp (RFC 3339).
    pub exported_at: String,
    /// The owner profile (effective value; a default profile is created when absent).
    pub profile: MyProfile,
    /// All persons owned by the user.
    pub persons: Vec<Person>,
    /// All owner→person relationship edges.
    pub relationships: Vec<Relationship>,
    /// All memories owned by the user.
    pub memories: Vec<Memory>,
    /// All stories owned by the user.
    pub stories: Vec<Story>,
    /// Currently derived reminders with dismiss/snooze state applied.
    pub reminders: Vec<Reminder>,
}
