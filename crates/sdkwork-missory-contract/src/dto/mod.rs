//! Wire DTOs for the Missory app-api surface.
//!
//! Wire rules (`API_SPEC.md` section 13.6): camelCase field names, `u64` ids
//! serialized as JSON strings, timestamps as RFC 3339 strings, optional fields
//! omitted when absent.

pub mod assistant;
pub mod memory;
pub mod person;
pub mod profile;
pub mod reminder;
pub mod story;

pub use assistant::{
    AssistantAnswer, AssistantQueryRequest, AssistantScenario, Briefing, BriefingRequest,
    ChatSummary, ChatSummaryRequest, Citation, CitationKind, HomeDigest, MessageDraft,
    MessageDraftRequest, MessageTone,
};
pub use memory::{
    Memory, MemoryExtractRequest, MemoryOrigin, MemoryStatus, MemoryType, MemoryUpsertRequest,
    SourceKind,
};
pub use person::{
    Importance, Person, PersonDetail, PersonDetailStats, PersonUpsertRequest, Relationship,
    RelationshipType, RelationshipUpsertRequest, TimelineEntry, TimelineEntryKind,
};
pub use profile::{MyProfile, MyProfileUpsertRequest};
pub use reminder::{Reminder, ReminderSnoozeRequest, ReminderType};
pub use story::{Story, StoryUpsertRequest};

use sdkwork_utils_rust::PageInfo;
use serde::{Deserialize, Serialize};

/// Standard offset page payload (`API_SPEC.md` section 16): `items` plus `pageInfo`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissoryPage<T> {
    /// Page items; never null, empty when no matches.
    pub items: Vec<T>,
    /// Pagination metadata (offset mode).
    pub page_info: PageInfo,
}
