//! Service port: every use case the Missory route layer may call.
//!
//! Route crates depend on this trait only — never on concrete services or stores
//! (`APPLICATION_LAYERED_ARCHITECTURE_SPEC.md` section 4.1).

use crate::context::MissoryRequestContext;
use crate::dto::{
    AssistantAnswer, AssistantQueryRequest, Briefing, BriefingRequest, ChatSummary,
    ChatSummaryRequest, DataExport, DataExportCreateRequest, HomeDigest, Memory,
    MemoryExtractRequest, MemoryOrigin, MemoryStatus, MemoryType, MemoryUpsertRequest,
    MessageDraft, MessageDraftRequest, MissoryPage, MyProfile, MyProfileUpsertRequest, Person,
    PersonDetail, PersonUpsertRequest, Relationship, RelationshipType, RelationshipUpsertRequest,
    Reminder, ReminderSnoozeRequest, ReminderType, Story, StoryUpsertRequest, TimelineEntry,
};
use crate::error::MissoryServiceResult;

/// Person list/search filters (`API_SPEC.md` section 14.1.1).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListPersonsQuery {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200).
    pub page_size: Option<i64>,
    /// Free-text keyword across name/aliases/tags/title/company/notes.
    pub q: Option<String>,
    /// Filter to persons whose relationship edge includes this type.
    pub relationship_type: Option<RelationshipType>,
}

/// Memory list/search filters.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListMemoriesQuery {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200).
    pub page_size: Option<i64>,
    /// Free-text keyword across title/content.
    pub q: Option<String>,
    /// Filter by person.
    pub person_id: Option<u64>,
    /// Filter by memory category.
    pub memory_type: Option<MemoryType>,
    /// Filter by lifecycle status.
    pub status: Option<MemoryStatus>,
    /// Filter by fact/inference origin.
    pub origin: Option<MemoryOrigin>,
}

/// Story list/search filters.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListStoriesQuery {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200).
    pub page_size: Option<i64>,
    /// Free-text keyword across title/summary/location.
    pub q: Option<String>,
}

/// Reminder list filters.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListRemindersQuery {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200).
    pub page_size: Option<i64>,
    /// Filter by reminder category.
    pub reminder_type: Option<ReminderType>,
}

/// Every Missory app-api use case (`apis/app-api/communication/missory-app-api.openapi.yaml`).
#[async_trait::async_trait]
pub trait MissoryAppApi: Send + Sync {
    // ---- my profile (PRD §11) ----

    /// Returns the owner profile, creating an empty default when missing.
    async fn get_my_profile(
        &self,
        context: &MissoryRequestContext,
    ) -> MissoryServiceResult<MyProfile>;

    /// Updates the owner profile with the provided fields.
    async fn update_my_profile(
        &self,
        context: &MissoryRequestContext,
        request: MyProfileUpsertRequest,
    ) -> MissoryServiceResult<MyProfile>;

    // ---- persons (PRD §12/§13) ----

    /// Creates a person (HTTP 201).
    async fn create_person(
        &self,
        context: &MissoryRequestContext,
        request: PersonUpsertRequest,
    ) -> MissoryServiceResult<Person>;

    /// Lists persons with filters and offset pagination.
    async fn list_persons(
        &self,
        context: &MissoryRequestContext,
        query: ListPersonsQuery,
    ) -> MissoryServiceResult<MissoryPage<Person>>;

    /// Returns the aggregated person detail view.
    async fn get_person(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
    ) -> MissoryServiceResult<PersonDetail>;

    /// Updates a person.
    async fn update_person(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
        request: PersonUpsertRequest,
    ) -> MissoryServiceResult<Person>;

    /// Deletes a person with its relationship edge and memory links.
    async fn delete_person(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
    ) -> MissoryServiceResult<()>;

    /// Returns the person's relationship timeline, newest first.
    async fn get_person_timeline(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
    ) -> MissoryServiceResult<MissoryPage<TimelineEntry>>;

    // ---- relationships (PRD §14) ----

    /// Creates or replaces the owner→person relationship edge (HTTP 201).
    async fn upsert_relationship(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
        request: RelationshipUpsertRequest,
    ) -> MissoryServiceResult<Relationship>;

    /// Removes the owner→person relationship edge.
    async fn delete_relationship(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
        relationship_id: u64,
    ) -> MissoryServiceResult<()>;

    // ---- memories (PRD §17/§18/§19) ----

    /// Records a manual memory (fact origin, HTTP 201).
    async fn create_memory(
        &self,
        context: &MissoryRequestContext,
        request: MemoryUpsertRequest,
    ) -> MissoryServiceResult<Memory>;

    /// Lists memories with filters and offset pagination.
    async fn list_memories(
        &self,
        context: &MissoryRequestContext,
        query: ListMemoriesQuery,
    ) -> MissoryServiceResult<MissoryPage<Memory>>;

    /// Returns one memory with source and inference metadata.
    async fn get_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<Memory>;

    /// Updates one memory.
    async fn update_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
        request: MemoryUpsertRequest,
    ) -> MissoryServiceResult<Memory>;

    /// Deletes one memory.
    async fn delete_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<()>;

    /// Confirms a candidate/inference memory into a user-confirmed fact (command).
    async fn confirm_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<Memory>;

    /// Rejects a candidate/inference memory (command).
    async fn reject_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<Memory>;

    /// Extracts candidate memories from free text (AI assist; candidates only).
    async fn extract_memories(
        &self,
        context: &MissoryRequestContext,
        request: MemoryExtractRequest,
    ) -> MissoryServiceResult<MissoryPage<Memory>>;

    // ---- stories (PRD §22) ----

    /// Creates a story (HTTP 201).
    async fn create_story(
        &self,
        context: &MissoryRequestContext,
        request: StoryUpsertRequest,
    ) -> MissoryServiceResult<Story>;

    /// Lists stories with filters and offset pagination.
    async fn list_stories(
        &self,
        context: &MissoryRequestContext,
        query: ListStoriesQuery,
    ) -> MissoryServiceResult<MissoryPage<Story>>;

    /// Returns one story.
    async fn get_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
    ) -> MissoryServiceResult<Story>;

    /// Updates one story.
    async fn update_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
        request: StoryUpsertRequest,
    ) -> MissoryServiceResult<Story>;

    /// Deletes one story.
    async fn delete_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
    ) -> MissoryServiceResult<()>;

    /// Generates (or regenerates) the AI story summary (draft; user-editable).
    async fn summarize_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
    ) -> MissoryServiceResult<Story>;

    // ---- reminders (PRD §27) ----

    /// Lists derived reminders with dismiss/snooze state applied.
    async fn list_reminders(
        &self,
        context: &MissoryRequestContext,
        query: ListRemindersQuery,
    ) -> MissoryServiceResult<MissoryPage<Reminder>>;

    /// Dismisses a derived reminder (command).
    async fn dismiss_reminder(
        &self,
        context: &MissoryRequestContext,
        reminder_id: &str,
    ) -> MissoryServiceResult<()>;

    /// Snoozes a derived reminder for `request.days` (command).
    async fn snooze_reminder(
        &self,
        context: &MissoryRequestContext,
        reminder_id: &str,
        request: ReminderSnoozeRequest,
    ) -> MissoryServiceResult<()>;

    // ---- home (PRD §10) ----

    /// Builds the home digest: today relations, recent memories, recent persons.
    async fn get_home_digest(
        &self,
        context: &MissoryRequestContext,
    ) -> MissoryServiceResult<HomeDigest>;

    // ---- privacy export (PRD §9: data deletion and export are supported) ----

    /// Creates a whole-account export document scoped to the request context.
    async fn create_data_export(
        &self,
        context: &MissoryRequestContext,
        request: DataExportCreateRequest,
    ) -> MissoryServiceResult<DataExport>;

    // ---- assistant (PRD §23–§26; draft-only) ----

    /// Answers a person/relationship/memory question with citations.
    async fn assistant_query(
        &self,
        context: &MissoryRequestContext,
        request: AssistantQueryRequest,
    ) -> MissoryServiceResult<AssistantAnswer>;

    /// Builds a meeting briefing for one person.
    async fn assistant_briefing(
        &self,
        context: &MissoryRequestContext,
        request: BriefingRequest,
    ) -> MissoryServiceResult<Briefing>;

    /// Drafts a message (birthday, check-in, ...). Never auto-sent.
    async fn assistant_message_draft(
        &self,
        context: &MissoryRequestContext,
        request: MessageDraftRequest,
    ) -> MissoryServiceResult<MessageDraft>;

    /// Summarizes pasted chat text and produces candidate memories.
    async fn assistant_chat_summary(
        &self,
        context: &MissoryRequestContext,
        request: ChatSummaryRequest,
    ) -> MissoryServiceResult<ChatSummary>;
}
