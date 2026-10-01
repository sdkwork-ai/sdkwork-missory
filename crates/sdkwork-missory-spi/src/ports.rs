//! The Missory store port and the pluggable social-text-model port.

use sdkwork_missory_contract::dto::{Memory, MyProfile, Person, Relationship, Story};
use sdkwork_missory_contract::ports::{ListMemoriesQuery, ListPersonsQuery, ListStoriesQuery};

use crate::error::MissoryStoreResult;

/// Owner isolation scope derived from the request context.
///
/// Every store operation is scoped to `(tenant_id, user_id)`; adapters must
/// enforce the scope on every read and write (strict data isolation, PRD §32.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissoryScope {
    /// Tenant id.
    pub tenant_id: u64,
    /// Owner user id.
    pub user_id: u64,
}

impl MissoryScope {
    /// Builds a scope from explicit ids.
    pub fn new(tenant_id: u64, user_id: u64) -> Self {
        Self { tenant_id, user_id }
    }
}

/// Persisted dismiss/snooze state for a derived reminder key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReminderStateRecord {
    /// Stable reminder key (for example `birthday:123`).
    pub reminder_id: String,
    /// Applied state.
    pub status: ReminderStateStatus,
    /// Snooze expiry (RFC 3339) when snoozed.
    pub snoozed_until: Option<String>,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Applied reminder state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderStateStatus {
    /// Dismissed permanently (until the underlying derivation changes period).
    Dismissed,
    /// Snoozed until [`ReminderStateRecord::snoozed_until`].
    Snoozed,
}

/// Persistence port for Missory aggregates.
///
/// Implementations must scope every operation by [`MissoryScope`], never leak
/// rows across owners, and treat `insert_*` ids as service-assigned snowflakes
/// (minted through [`MissoryStore::next_id`]).
#[async_trait::async_trait]
pub trait MissoryStore: Send + Sync {
    /// Mints the next process-unique positive id.
    async fn next_id(&self) -> MissoryStoreResult<u64>;

    // ---- my profile ----

    /// Returns the owner profile when present.
    async fn get_my_profile(&self, scope: &MissoryScope) -> MissoryStoreResult<Option<MyProfile>>;

    /// Inserts or replaces the owner profile.
    async fn put_my_profile(
        &self,
        scope: &MissoryScope,
        profile: MyProfile,
    ) -> MissoryStoreResult<MyProfile>;

    // ---- persons ----

    /// Inserts a person record.
    async fn insert_person(
        &self,
        scope: &MissoryScope,
        person: Person,
    ) -> MissoryStoreResult<Person>;

    /// Replaces a person record by id.
    async fn update_person(
        &self,
        scope: &MissoryScope,
        person: Person,
    ) -> MissoryStoreResult<Person>;

    /// Deletes a person and cascades its memories/story links.
    async fn delete_person(&self, scope: &MissoryScope, person_id: u64) -> MissoryStoreResult<()>;

    /// Fetches one person, or `None` when absent.
    async fn get_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<Option<Person>>;

    /// Lists persons matching the filters with the total match count.
    async fn list_persons(
        &self,
        scope: &MissoryScope,
        query: ListPersonsQuery,
    ) -> MissoryStoreResult<(Vec<Person>, i64)>;

    /// Returns the most recently touched persons, newest first.
    async fn recent_persons(
        &self,
        scope: &MissoryScope,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Person>>;

    // ---- relationships ----

    /// Inserts or replaces the owner→person relationship edge.
    async fn upsert_relationship(
        &self,
        scope: &MissoryScope,
        relationship: Relationship,
    ) -> MissoryStoreResult<Relationship>;

    /// Deletes the owner→person relationship edge.
    async fn delete_relationship(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<()>;

    /// Fetches the owner→person relationship edge, or `None`.
    async fn get_relationship(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<Option<Relationship>>;

    /// Fetches a relationship edge by its own id, or `None`.
    async fn get_relationship_by_id(
        &self,
        scope: &MissoryScope,
        relationship_id: u64,
    ) -> MissoryStoreResult<Option<Relationship>>;

    // ---- memories ----

    /// Inserts a memory record.
    async fn insert_memory(
        &self,
        scope: &MissoryScope,
        memory: Memory,
    ) -> MissoryStoreResult<Memory>;

    /// Replaces a memory record by id.
    async fn update_memory(
        &self,
        scope: &MissoryScope,
        memory: Memory,
    ) -> MissoryStoreResult<Memory>;

    /// Deletes a memory record.
    async fn delete_memory(&self, scope: &MissoryScope, memory_id: u64) -> MissoryStoreResult<()>;

    /// Fetches one memory, or `None` when absent.
    async fn get_memory(
        &self,
        scope: &MissoryScope,
        memory_id: u64,
    ) -> MissoryStoreResult<Option<Memory>>;

    /// Lists memories matching the filters with the total match count.
    async fn list_memories(
        &self,
        scope: &MissoryScope,
        query: ListMemoriesQuery,
    ) -> MissoryStoreResult<(Vec<Memory>, i64)>;

    /// Returns the most recently created memories, newest first.
    async fn recent_memories(
        &self,
        scope: &MissoryScope,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Memory>>;

    /// Returns memories for one person, newest first.
    async fn memories_for_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Memory>>;

    // ---- stories ----

    /// Inserts a story record.
    async fn insert_story(&self, scope: &MissoryScope, story: Story) -> MissoryStoreResult<Story>;

    /// Replaces a story record by id.
    async fn update_story(&self, scope: &MissoryScope, story: Story) -> MissoryStoreResult<Story>;

    /// Deletes a story record.
    async fn delete_story(&self, scope: &MissoryScope, story_id: u64) -> MissoryStoreResult<()>;

    /// Fetches one story, or `None` when absent.
    async fn get_story(
        &self,
        scope: &MissoryScope,
        story_id: u64,
    ) -> MissoryStoreResult<Option<Story>>;

    /// Lists stories matching the filters with the total match count.
    async fn list_stories(
        &self,
        scope: &MissoryScope,
        query: ListStoriesQuery,
    ) -> MissoryStoreResult<(Vec<Story>, i64)>;

    /// Returns stories including the given person, newest first.
    async fn stories_for_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Story>>;

    // ---- reminder states ----

    /// Returns the applied state for a reminder key, or `None`.
    async fn get_reminder_state(
        &self,
        scope: &MissoryScope,
        reminder_id: &str,
    ) -> MissoryStoreResult<Option<ReminderStateRecord>>;

    /// Persists the applied state for a reminder key.
    async fn put_reminder_state(
        &self,
        scope: &MissoryScope,
        record: ReminderStateRecord,
    ) -> MissoryStoreResult<ReminderStateRecord>;
}

/// One text-generation request for the pluggable language model port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocialTextPrompt {
    /// What to produce (task description).
    pub instruction: String,
    /// Structured context the model may use (never other users' data).
    pub context: String,
}

/// Pluggable language-model port (PRD §39: AI Provider 可插拔).
///
/// The rule-based assistant works without any implementation; when one is
/// configured, the assistant may refine deterministic drafts through it and must
/// fall back to the deterministic text on error.
#[async_trait::async_trait]
pub trait SocialTextModel: Send + Sync {
    /// Produces text for the prompt.
    async fn complete(&self, prompt: &SocialTextPrompt) -> MissoryStoreResult<String>;
}
