//! Reminder DTOs (PRD §27).

use sdkwork_utils_rust::serde_uint64;
use serde::{Deserialize, Serialize};

/// Derived reminder type (PRD §27).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReminderType {
    /// No contact within the relationship cycle.
    LongUncontacted,
    /// Birthday approaching.
    Birthday,
    /// Open commitment (promise).
    Commitment,
    /// Important life event or change.
    ImportantEvent,
    /// Anniversary of a story.
    Anniversary,
}

/// A derived relationship reminder.
///
/// Reminders are computed from people/relationships/memories/stories; the id is a
/// stable string key (`birthday:123`) used by the dismiss/snooze commands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    /// Stable reminder key (for example `birthday:123`).
    pub reminder_id: String,
    /// Reminder category.
    #[serde(rename = "type")]
    pub reminder_type: ReminderType,
    /// Related person id (wire: string).
    #[serde(
        serialize_with = "serde_uint64::serialize",
        deserialize_with = "serde_uint64::deserialize"
    )]
    pub person_id: u64,
    /// Related person display name.
    pub person_name: String,
    /// Human title (for example `生日还有 3 天`).
    pub title: String,
    /// Optional detail text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// When the reminder is due (RFC 3339).
    pub due_at: String,
    /// Days the reminder is overdue (long-uncontacted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_overdue: Option<i64>,
    /// Days until the due date (birthday/anniversary).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_until: Option<i64>,
}

/// Snooze command body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderSnoozeRequest {
    /// Snooze duration in days (1..=365).
    pub days: i64,
}
