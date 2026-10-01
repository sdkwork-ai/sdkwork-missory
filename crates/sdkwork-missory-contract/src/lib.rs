//! HTTP contracts and service ports for SDKWork Missory.
//!
//! This crate is the L2/L3 contract boundary: DTOs use the SDKWork wire rules
//! (camelCase JSON, `u64` ids serialized as strings), the [`ports::MissoryAppApi`]
//! trait defines every use case the route layer may call, and
//! [`error::MissoryServiceError`] is the only error type crossing that boundary.

pub mod context;
pub mod dto;
pub mod error;
pub mod ports;
pub mod wire;

pub use context::MissoryRequestContext;
pub use dto::{
    AssistantAnswer, AssistantQueryRequest, AssistantScenario, Briefing, BriefingRequest,
    ChatSummary, ChatSummaryRequest, Citation, CitationKind, HomeDigest, Importance, Memory,
    MemoryExtractRequest, MemoryOrigin, MemoryStatus, MemoryType, MemoryUpsertRequest,
    MessageDraft, MessageDraftRequest, MessageTone, MissoryPage, MyProfile, MyProfileUpsertRequest,
    Person, PersonDetail, PersonDetailStats, PersonUpsertRequest, Relationship, RelationshipType,
    RelationshipUpsertRequest, Reminder, ReminderSnoozeRequest, ReminderType, SourceKind, Story,
    StoryUpsertRequest, TimelineEntry, TimelineEntryKind,
};
pub use error::{MissoryServiceError, MissoryServiceErrorKind, MissoryServiceResult};
pub use ports::MissoryAppApi;
pub use sdkwork_utils_rust::PageInfo;
