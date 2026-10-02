//! `MissoryService`: the concrete `MissoryAppApi` implementation.

use std::sync::Arc;

use sdkwork_missory_contract::context::MissoryRequestContext;
use sdkwork_missory_contract::dto::{
    AssistantAnswer, AssistantQueryRequest, Briefing, BriefingRequest, ChatSummary,
    ChatSummaryRequest, DataExport, DataExportCreateRequest, HomeDigest, Importance, Memory,
    MemoryExtractRequest, MemoryOrigin, MemoryStatus, MemoryType, MemoryUpsertRequest,
    MessageDraft, MessageDraftRequest, MissoryPage, MyProfile, MyProfileUpsertRequest, Person,
    PersonDetail, PersonDetailStats, PersonUpsertRequest, Relationship, RelationshipUpsertRequest,
    Reminder, ReminderSnoozeRequest, Story, StoryUpsertRequest, TimelineEntry, TimelineEntryKind,
};
use sdkwork_missory_contract::error::{
    MissoryServiceError, MissoryServiceErrorKind, MissoryServiceResult,
};
use sdkwork_missory_contract::ports::{
    ListMemoriesQuery, ListPersonsQuery, ListRemindersQuery, ListStoriesQuery, MissoryAppApi,
};
use sdkwork_missory_spi::{MissoryScope, MissoryStore, SocialTextModel};
use sdkwork_utils_rust::{
    offset_list_page_info, validated_offset_list_params, OffsetListPageParams, PageInfo,
};

use crate::assistant;
use crate::extract;
use crate::reminders;

/// Default long-uncontacted cycle when a relationship declares no custom cycle.
pub const DEFAULT_LONG_UNCONTACTED_DAYS: i64 = 30;

/// Injectable clock for deterministic tests.
pub type Clock = Arc<dyn Fn() -> time::OffsetDateTime + Send + Sync>;

/// The Missory domain service.
pub struct MissoryService {
    store: Arc<dyn MissoryStore>,
    model: Option<Arc<dyn SocialTextModel>>,
    clock: Clock,
}

impl MissoryService {
    /// Builds a service over a store with an optional language model.
    pub fn new(store: Arc<dyn MissoryStore>, model: Option<Arc<dyn SocialTextModel>>) -> Self {
        Self {
            store,
            model,
            clock: Arc::new(time::OffsetDateTime::now_utc),
        }
    }

    /// Replaces the clock (test seam).
    #[must_use]
    pub fn with_clock(mut self, clock: Clock) -> Self {
        self.clock = clock;
        self
    }

    fn now(&self) -> time::OffsetDateTime {
        (self.clock)()
    }

    fn now_rfc3339(&self) -> String {
        format_rfc3339(self.now())
    }

    fn scope(context: &MissoryRequestContext) -> MissoryScope {
        MissoryScope::new(context.tenant_id, context.user_id)
    }

    fn map_store(error: sdkwork_missory_spi::MissoryStoreError) -> MissoryServiceError {
        match error {
            sdkwork_missory_spi::MissoryStoreError::NotFound(detail) => {
                MissoryServiceError::new(MissoryServiceErrorKind::NotFound, "not_found", detail)
            }
            sdkwork_missory_spi::MissoryStoreError::Conflict(detail) => {
                MissoryServiceError::conflict(detail)
            }
            sdkwork_missory_spi::MissoryStoreError::Storage(detail) => {
                tracing::error!(detail = %detail, "missory store failure");
                MissoryServiceError::storage("storage operation failed")
            }
        }
    }

    fn page_params(
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> MissoryServiceResult<OffsetListPageParams> {
        validated_offset_list_params(page, page_size)
            .map_err(|_| MissoryServiceError::invalid_parameter("page or page_size out of range"))
    }

    fn page<T>(items: Vec<T>, total: i64, params: &OffsetListPageParams) -> MissoryPage<T> {
        MissoryPage {
            items,
            page_info: offset_page_info(total, params),
        }
    }
}

impl MissoryService {
    /// Walks every person page under the scope (whole-collection export walk).
    async fn collect_person_pages(
        &self,
        scope: &MissoryScope,
    ) -> MissoryServiceResult<Vec<Person>> {
        let mut collected = Vec::new();
        let mut page = 1i64;
        loop {
            let params = Self::page_params(Some(page), Some(EXPORT_PAGE_SIZE))?;
            let (items, total) = self
                .store
                .list_persons(
                    scope,
                    ListPersonsQuery {
                        page: Some(params.page),
                        page_size: Some(params.page_size),
                        ..ListPersonsQuery::default()
                    },
                )
                .await
                .map_err(Self::map_store)?;
            collected.extend(items);
            if params.page * params.page_size >= total {
                return Ok(collected);
            }
            page += 1;
        }
    }

    /// Walks every memory page under the scope (whole-collection export walk).
    async fn collect_memory_pages(
        &self,
        scope: &MissoryScope,
    ) -> MissoryServiceResult<Vec<Memory>> {
        let mut collected = Vec::new();
        let mut page = 1i64;
        loop {
            let params = Self::page_params(Some(page), Some(EXPORT_PAGE_SIZE))?;
            let (items, total) = self
                .store
                .list_memories(
                    scope,
                    ListMemoriesQuery {
                        page: Some(params.page),
                        page_size: Some(params.page_size),
                        ..ListMemoriesQuery::default()
                    },
                )
                .await
                .map_err(Self::map_store)?;
            collected.extend(items);
            if params.page * params.page_size >= total {
                return Ok(collected);
            }
            page += 1;
        }
    }

    /// Walks every story page under the scope (whole-collection export walk).
    async fn collect_story_pages(
        &self,
        scope: &MissoryScope,
    ) -> MissoryServiceResult<Vec<Story>> {
        let mut collected = Vec::new();
        let mut page = 1i64;
        loop {
            let params = Self::page_params(Some(page), Some(EXPORT_PAGE_SIZE))?;
            let (items, total) = self
                .store
                .list_stories(
                    scope,
                    ListStoriesQuery {
                        page: Some(params.page),
                        page_size: Some(params.page_size),
                        ..ListStoriesQuery::default()
                    },
                )
                .await
                .map_err(Self::map_store)?;
            collected.extend(items);
            if params.page * params.page_size >= total {
                return Ok(collected);
            }
            page += 1;
        }
    }
}

/// Formats an instant as an RFC 3339 UTC timestamp.
pub fn format_rfc3339(instant: time::OffsetDateTime) -> String {
    instant
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

/// Builds offset pagination metadata from validated params.
pub fn offset_page_info(total: i64, params: &OffsetListPageParams) -> PageInfo {
    offset_list_page_info(total, *params)
}

/// Parses an RFC 3339 timestamp, returning `None` for invalid input.
pub fn parse_rfc3339(value: &str) -> Option<time::OffsetDateTime> {
    time::OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339).ok()
}

fn person_days_between(later: time::OffsetDateTime, earlier: time::OffsetDateTime) -> i64 {
    let seconds = (later - earlier).whole_seconds();
    if seconds <= 0 {
        0
    } else {
        seconds / 86_400
    }
}

fn validate_person_request(request: &PersonUpsertRequest) -> MissoryServiceResult<()> {
    let name = request.display_name.trim();
    if name.is_empty() {
        return Err(MissoryServiceError::validation("displayName is required"));
    }
    if name.chars().count() > 120 {
        return Err(MissoryServiceError::validation(
            "displayName must be at most 120 characters",
        ));
    }
    if let Some(birthday) = request.birthday.as_deref() {
        if !reminders::is_valid_birthday(birthday) {
            return Err(MissoryServiceError::invalid_parameter(
                "birthday must be MM-DD or YYYY-MM-DD",
            ));
        }
    }
    Ok(())
}

fn validate_memory_request(request: &MemoryUpsertRequest) -> MissoryServiceResult<()> {
    let content = request.content.trim();
    if content.is_empty() {
        return Err(MissoryServiceError::validation("content is required"));
    }
    if content.chars().count() > 4000 {
        return Err(MissoryServiceError::validation(
            "content must be at most 4000 characters",
        ));
    }
    if let Some(occurred_at) = request.occurred_at.as_deref() {
        if parse_rfc3339(occurred_at).is_none() {
            return Err(MissoryServiceError::invalid_parameter(
                "occurredAt must be an RFC 3339 timestamp",
            ));
        }
    }
    Ok(())
}

fn apply_person_fields(person: &mut Person, request: &PersonUpsertRequest) {
    person.display_name = request.display_name.trim().to_string();
    person.aliases = request
        .aliases
        .clone()
        .unwrap_or_else(|| person.aliases.clone());
    person.gender = request.gender.clone().or_else(|| person.gender.clone());
    person.birthday = request.birthday.clone().or_else(|| person.birthday.clone());
    person.city = request.city.clone().or_else(|| person.city.clone());
    person.title = request.title.clone().or_else(|| person.title.clone());
    person.company = request.company.clone().or_else(|| person.company.clone());
    person.avatar_url = request
        .avatar_url
        .clone()
        .or_else(|| person.avatar_url.clone());
    person.tags = request.tags.clone().unwrap_or_else(|| person.tags.clone());
    person.interests = request
        .interests
        .clone()
        .unwrap_or_else(|| person.interests.clone());
    person.preferences = request
        .preferences
        .clone()
        .unwrap_or_else(|| person.preferences.clone());
    person.bio = request.bio.clone().or_else(|| person.bio.clone());
    person.contact_channels = request
        .contact_channels
        .clone()
        .unwrap_or_else(|| person.contact_channels.clone());
    person.notes = request.notes.clone().or_else(|| person.notes.clone());
}

#[async_trait::async_trait]
impl MissoryAppApi for MissoryService {
    async fn get_my_profile(
        &self,
        context: &MissoryRequestContext,
    ) -> MissoryServiceResult<MyProfile> {
        let scope = Self::scope(context);
        if let Some(profile) = self
            .store
            .get_my_profile(&scope)
            .await
            .map_err(Self::map_store)?
        {
            return Ok(profile);
        }
        let now = self.now_rfc3339();
        let profile = MyProfile {
            user_id: context.user_id,
            display_name: String::new(),
            nickname: None,
            city: None,
            occupation: None,
            company: None,
            education: None,
            interests: vec![],
            likes: vec![],
            dislikes: vec![],
            communication_style: None,
            bio: None,
            created_at: now.clone(),
            updated_at: now,
        };
        self.store
            .put_my_profile(&scope, profile.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(profile)
    }

    async fn update_my_profile(
        &self,
        context: &MissoryRequestContext,
        request: MyProfileUpsertRequest,
    ) -> MissoryServiceResult<MyProfile> {
        let mut profile = self.get_my_profile(context).await?;
        if let Some(value) = request.display_name {
            profile.display_name = value;
        }
        if let Some(value) = request.nickname {
            profile.nickname = Some(value);
        }
        if let Some(value) = request.city {
            profile.city = Some(value);
        }
        if let Some(value) = request.occupation {
            profile.occupation = Some(value);
        }
        if let Some(value) = request.company {
            profile.company = Some(value);
        }
        if let Some(value) = request.education {
            profile.education = Some(value);
        }
        if let Some(value) = request.interests {
            profile.interests = value;
        }
        if let Some(value) = request.likes {
            profile.likes = value;
        }
        if let Some(value) = request.dislikes {
            profile.dislikes = value;
        }
        if let Some(value) = request.communication_style {
            profile.communication_style = Some(value);
        }
        if let Some(value) = request.bio {
            profile.bio = Some(value);
        }
        profile.user_id = context.user_id;
        profile.updated_at = self.now_rfc3339();
        self.store
            .put_my_profile(&Self::scope(context), profile.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(profile)
    }

    async fn create_person(
        &self,
        context: &MissoryRequestContext,
        request: PersonUpsertRequest,
    ) -> MissoryServiceResult<Person> {
        validate_person_request(&request)?;
        let scope = Self::scope(context);
        let now = self.now_rfc3339();
        let id = self.store.next_id().await.map_err(Self::map_store)?;
        let mut person = Person {
            id,
            display_name: String::new(),
            aliases: vec![],
            gender: None,
            birthday: None,
            city: None,
            title: None,
            company: None,
            avatar_url: None,
            tags: vec![],
            interests: vec![],
            preferences: vec![],
            bio: None,
            contact_channels: std::collections::BTreeMap::new(),
            notes: None,
            last_contacted_at: None,
            created_at: now.clone(),
            updated_at: now,
        };
        apply_person_fields(&mut person, &request);
        let person = self
            .store
            .insert_person(&scope, person)
            .await
            .map_err(Self::map_store)?;
        if let Some(types) = request.relationship_types.clone() {
            if !types.is_empty() {
                let relationship_id = self.store.next_id().await.map_err(Self::map_store)?;
                self.store
                    .upsert_relationship(
                        &scope,
                        Relationship {
                            id: relationship_id,
                            person_id: person.id,
                            relationship_types: types,
                            started_at: Some(self.now_rfc3339()),
                            last_contacted_at: None,
                            description: None,
                            importance: Some(Importance::Normal),
                            contact_cycle_days: None,
                            notes: None,
                            created_at: self.now_rfc3339(),
                            updated_at: self.now_rfc3339(),
                        },
                    )
                    .await
                    .map_err(Self::map_store)?;
            }
        }
        Ok(person)
    }

    async fn list_persons(
        &self,
        context: &MissoryRequestContext,
        query: ListPersonsQuery,
    ) -> MissoryServiceResult<MissoryPage<Person>> {
        let params = Self::page_params(query.page, query.page_size)?;
        let mut store_query = query;
        store_query.page = Some(params.page);
        store_query.page_size = Some(params.page_size);
        let (items, total) = self
            .store
            .list_persons(&Self::scope(context), store_query)
            .await
            .map_err(Self::map_store)?;
        Ok(Self::page(items, total, &params))
    }

    async fn get_person(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
    ) -> MissoryServiceResult<PersonDetail> {
        let scope = Self::scope(context);
        let person = self
            .store
            .get_person(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("person", person_id))?;
        let relationship = self
            .store
            .get_relationship(&scope, person_id)
            .await
            .map_err(Self::map_store)?;
        let recent_memories = self
            .store
            .memories_for_person(&scope, person_id, 5)
            .await
            .map_err(Self::map_store)?;
        let commitments = self
            .store
            .memories_for_person(&scope, person_id, 50)
            .await
            .map_err(Self::map_store)?
            .into_iter()
            .filter(|memory| {
                memory.memory_type == MemoryType::Commitment
                    && !matches!(
                        memory.status,
                        MemoryStatus::Rejected | MemoryStatus::Archived | MemoryStatus::Expired
                    )
            })
            .take(5)
            .collect::<Vec<_>>();
        let stories = self
            .store
            .stories_for_person(&scope, person_id, 5)
            .await
            .map_err(Self::map_store)?;
        let memory_count = self
            .store
            .memories_for_person(&scope, person_id, usize::MAX >> 4)
            .await
            .map_err(Self::map_store)?
            .len() as i64;
        let story_count = self
            .store
            .stories_for_person(&scope, person_id, usize::MAX >> 4)
            .await
            .map_err(Self::map_store)?
            .len() as i64;
        let now = self.now();
        let known_anchor = relationship
            .as_ref()
            .and_then(|edge| edge.started_at.as_deref())
            .and_then(parse_rfc3339)
            .or_else(|| parse_rfc3339(&person.created_at))
            .unwrap_or(now);
        let last_contact = relationship
            .as_ref()
            .and_then(|edge| edge.last_contacted_at.as_deref())
            .or(person.last_contacted_at.as_deref())
            .and_then(parse_rfc3339);
        Ok(PersonDetail {
            relationships: relationship.into_iter().collect::<Vec<_>>(),
            recent_memories,
            commitments,
            stories,
            stats: PersonDetailStats {
                memory_count,
                story_count,
                known_days: person_days_between(now, known_anchor),
                days_since_last_contact: last_contact
                    .map(|instant| person_days_between(now, instant)),
            },
            person,
        })
    }

    async fn update_person(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
        request: PersonUpsertRequest,
    ) -> MissoryServiceResult<Person> {
        validate_person_request(&request)?;
        let scope = Self::scope(context);
        let mut person = self
            .store
            .get_person(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("person", person_id))?;
        apply_person_fields(&mut person, &request);
        person.updated_at = self.now_rfc3339();
        self.store
            .update_person(&scope, person.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(person)
    }

    async fn delete_person(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
    ) -> MissoryServiceResult<()> {
        let scope = Self::scope(context);
        self.store
            .get_person(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("person", person_id))?;
        self.store
            .delete_person(&scope, person_id)
            .await
            .map_err(Self::map_store)?;
        Ok(())
    }

    async fn get_person_timeline(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
    ) -> MissoryServiceResult<MissoryPage<TimelineEntry>> {
        let params = Self::page_params(None, None)?;
        let scope = Self::scope(context);
        let person = self
            .store
            .get_person(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("person", person_id))?;
        let relationship = self
            .store
            .get_relationship(&scope, person_id)
            .await
            .map_err(Self::map_store)?;
        let mut entries: Vec<TimelineEntry> = Vec::new();
        if let Some(edge) = relationship.as_ref() {
            if let Some(started_at) = edge.started_at.as_deref() {
                entries.push(TimelineEntry {
                    occurred_at: started_at.to_string(),
                    kind: TimelineEntryKind::RelationshipStart,
                    title: "第一次认识".to_string(),
                    detail: edge.description.clone(),
                    memory_id: None,
                    story_id: None,
                });
            }
            if let Some(last_contacted_at) = edge.last_contacted_at.as_deref() {
                entries.push(TimelineEntry {
                    occurred_at: last_contacted_at.to_string(),
                    kind: TimelineEntryKind::Contact,
                    title: "最近一次联系".to_string(),
                    detail: None,
                    memory_id: None,
                    story_id: None,
                });
            }
        }
        for memory in self
            .store
            .memories_for_person(&scope, person_id, usize::MAX >> 4)
            .await
            .map_err(Self::map_store)?
        {
            let occurred_at = memory
                .occurred_at
                .clone()
                .unwrap_or_else(|| memory.created_at.clone());
            entries.push(TimelineEntry {
                occurred_at,
                kind: TimelineEntryKind::Memory,
                title: memory
                    .title
                    .clone()
                    .unwrap_or_else(|| memory.content.chars().take(24).collect::<String>()),
                detail: Some(memory.content),
                memory_id: Some(memory.id),
                story_id: None,
            });
        }
        for story in self
            .store
            .stories_for_person(&scope, person_id, usize::MAX >> 4)
            .await
            .map_err(Self::map_store)?
        {
            let occurred_at = story
                .started_at
                .clone()
                .unwrap_or_else(|| story.created_at.clone());
            entries.push(TimelineEntry {
                occurred_at,
                kind: TimelineEntryKind::Story,
                title: story.title.clone(),
                detail: story.summary.clone(),
                memory_id: None,
                story_id: Some(story.id),
            });
        }
        entries.sort_by(|left, right| right.occurred_at.cmp(&left.occurred_at));
        let total = entries.len() as i64;
        let _ = &person;
        Ok(Self::page(entries, total, &params))
    }

    async fn upsert_relationship(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
        request: RelationshipUpsertRequest,
    ) -> MissoryServiceResult<Relationship> {
        if request.relationship_types.is_empty() {
            return Err(MissoryServiceError::validation(
                "at least one relationship type is required",
            ));
        }
        let scope = Self::scope(context);
        let person = self
            .store
            .get_person(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("person", person_id))?;
        let now = self.now_rfc3339();
        let mut relationship = self
            .store
            .get_relationship(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .unwrap_or_else(|| Relationship {
                id: 0,
                person_id,
                relationship_types: vec![],
                started_at: None,
                last_contacted_at: None,
                description: None,
                importance: None,
                contact_cycle_days: None,
                notes: None,
                created_at: now.clone(),
                updated_at: now.clone(),
            });
        if relationship.id == 0 {
            relationship.id = self.store.next_id().await.map_err(Self::map_store)?;
        }
        relationship.relationship_types = request.relationship_types;
        if request.started_at.is_some() {
            relationship.started_at = request.started_at.clone();
        }
        if request.last_contacted_at.is_some() {
            relationship.last_contacted_at = request.last_contacted_at.clone();
        }
        if request.description.is_some() {
            relationship.description = request.description.clone();
        }
        if request.importance.is_some() {
            relationship.importance = request.importance;
        }
        if request.contact_cycle_days.is_some() {
            relationship.contact_cycle_days = request.contact_cycle_days;
        }
        if request.notes.is_some() {
            relationship.notes = request.notes.clone();
        }
        relationship.updated_at = now.clone();
        let saved = self
            .store
            .upsert_relationship(&scope, relationship.clone())
            .await
            .map_err(Self::map_store)?;
        if let Some(last_contacted_at) = request.last_contacted_at.clone() {
            let mut person_update = person;
            person_update.last_contacted_at = Some(last_contacted_at);
            person_update.updated_at = now;
            self.store
                .update_person(&scope, person_update)
                .await
                .map_err(Self::map_store)?;
        }
        Ok(saved)
    }

    async fn delete_relationship(
        &self,
        context: &MissoryRequestContext,
        person_id: u64,
        relationship_id: u64,
    ) -> MissoryServiceResult<()> {
        let scope = Self::scope(context);
        let edge = self
            .store
            .get_relationship(&scope, person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("relationship", relationship_id))?;
        if edge.id != relationship_id {
            return Err(MissoryServiceError::not_found(
                "relationship",
                relationship_id,
            ));
        }
        self.store
            .delete_relationship(&scope, person_id)
            .await
            .map_err(Self::map_store)?;
        Ok(())
    }

    async fn create_memory(
        &self,
        context: &MissoryRequestContext,
        request: MemoryUpsertRequest,
    ) -> MissoryServiceResult<Memory> {
        validate_memory_request(&request)?;
        let scope = Self::scope(context);
        if self
            .store
            .get_person(&scope, request.person_id)
            .await
            .map_err(Self::map_store)?
            .is_none()
        {
            return Err(MissoryServiceError::not_found("person", request.person_id));
        }
        let now = self.now_rfc3339();
        let memory = Memory {
            id: self.store.next_id().await.map_err(Self::map_store)?,
            person_id: request.person_id,
            story_id: request.story_id,
            memory_type: request.memory_type,
            title: request.title,
            content: request.content.trim().to_string(),
            origin: MemoryOrigin::Fact,
            status: MemoryStatus::Confirmed,
            confidence: None,
            source_reason: None,
            source_kind: sdkwork_missory_contract::dto::SourceKind::or_user_input(
                request.source_kind,
            ),
            source_ref: request.source_ref,
            importance: Some(Importance::or_normal(request.importance)),
            occurred_at: request.occurred_at,
            created_at: now.clone(),
            updated_at: now,
        };
        self.store
            .insert_memory(&scope, memory.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(memory)
    }

    async fn list_memories(
        &self,
        context: &MissoryRequestContext,
        query: ListMemoriesQuery,
    ) -> MissoryServiceResult<MissoryPage<Memory>> {
        let params = Self::page_params(query.page, query.page_size)?;
        let mut store_query = query;
        store_query.page = Some(params.page);
        store_query.page_size = Some(params.page_size);
        let (items, total) = self
            .store
            .list_memories(&Self::scope(context), store_query)
            .await
            .map_err(Self::map_store)?;
        Ok(Self::page(items, total, &params))
    }

    async fn get_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<Memory> {
        self.store
            .get_memory(&Self::scope(context), memory_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("memory", memory_id))
    }

    async fn update_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
        request: MemoryUpsertRequest,
    ) -> MissoryServiceResult<Memory> {
        validate_memory_request(&request)?;
        let scope = Self::scope(context);
        let mut memory = self
            .store
            .get_memory(&scope, memory_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("memory", memory_id))?;
        memory.person_id = request.person_id;
        memory.story_id = request.story_id;
        memory.memory_type = request.memory_type;
        memory.title = request.title;
        memory.content = request.content.trim().to_string();
        memory.importance = Some(Importance::or_normal(request.importance));
        if request.occurred_at.is_some() {
            memory.occurred_at = request.occurred_at;
        }
        if request.source_kind.is_some() {
            memory.source_kind =
                sdkwork_missory_contract::dto::SourceKind::or_user_input(request.source_kind);
        }
        memory.source_ref = request.source_ref.or(memory.source_ref);
        memory.updated_at = self.now_rfc3339();
        self.store
            .update_memory(&scope, memory.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(memory)
    }

    async fn delete_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<()> {
        let scope = Self::scope(context);
        self.store
            .get_memory(&scope, memory_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("memory", memory_id))?;
        self.store
            .delete_memory(&scope, memory_id)
            .await
            .map_err(Self::map_store)?;
        Ok(())
    }

    async fn confirm_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<Memory> {
        let scope = Self::scope(context);
        let mut memory = self
            .store
            .get_memory(&scope, memory_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("memory", memory_id))?;
        if matches!(
            memory.status,
            MemoryStatus::Confirmed | MemoryStatus::Rejected | MemoryStatus::Expired
        ) {
            return Err(MissoryServiceError::conflict(
                "only candidate memories can be confirmed",
            ));
        }
        // Explicit user confirmation is the only path that promotes a record to a
        // user-confirmed fact (PRD §19: no silent AI promotion).
        memory.origin = MemoryOrigin::Fact;
        memory.status = MemoryStatus::Confirmed;
        memory.confidence = None;
        memory.source_kind = sdkwork_missory_contract::dto::SourceKind::UserInput;
        memory.updated_at = self.now_rfc3339();
        self.store
            .update_memory(&scope, memory.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(memory)
    }

    async fn reject_memory(
        &self,
        context: &MissoryRequestContext,
        memory_id: u64,
    ) -> MissoryServiceResult<Memory> {
        let scope = Self::scope(context);
        let mut memory = self
            .store
            .get_memory(&scope, memory_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("memory", memory_id))?;
        if memory.status != MemoryStatus::Candidate {
            return Err(MissoryServiceError::conflict(
                "only candidate memories can be rejected",
            ));
        }
        memory.status = MemoryStatus::Rejected;
        memory.updated_at = self.now_rfc3339();
        self.store
            .update_memory(&scope, memory.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(memory)
    }

    async fn extract_memories(
        &self,
        context: &MissoryRequestContext,
        request: MemoryExtractRequest,
    ) -> MissoryServiceResult<MissoryPage<Memory>> {
        let scope = Self::scope(context);
        let text = request.text.trim().to_string();
        if text.is_empty() {
            return Err(MissoryServiceError::validation("text is required"));
        }
        if text.chars().count() > 20_000 {
            return Err(MissoryServiceError::validation(
                "text must be at most 20000 characters",
            ));
        }
        let person_id = match request.person_id {
            Some(id) => {
                if self
                    .store
                    .get_person(&scope, id)
                    .await
                    .map_err(Self::map_store)?
                    .is_none()
                {
                    return Err(MissoryServiceError::not_found("person", id));
                }
                Some(id)
            }
            None => extract::resolve_person_by_mention(&text, &self.store, &scope)
                .await
                .map_err(Self::map_store)?,
        };
        let Some(person_id) = person_id else {
            return Ok(Self::page(
                vec![],
                0,
                &Self::page_params(Some(1), Some(20))?,
            ));
        };
        let now = self.now();
        let candidates = extract::extract_candidates(person_id, &text, now);
        let mut created = Vec::with_capacity(candidates.len());
        for mut candidate in candidates {
            let exists = self
                .store
                .memories_for_person(&scope, person_id, usize::MAX >> 4)
                .await
                .map_err(Self::map_store)?
                .iter()
                .any(|memory| memory.content == candidate.content);
            if exists {
                continue;
            }
            candidate.id = self.store.next_id().await.map_err(Self::map_store)?;
            candidate.created_at = self.now_rfc3339();
            candidate.updated_at = self.now_rfc3339();
            let saved = self
                .store
                .insert_memory(&scope, candidate)
                .await
                .map_err(Self::map_store)?;
            created.push(saved);
        }
        let total = created.len() as i64;
        Ok(Self::page(
            created,
            total,
            &Self::page_params(Some(1), Some(20))?,
        ))
    }

    async fn create_story(
        &self,
        context: &MissoryRequestContext,
        request: StoryUpsertRequest,
    ) -> MissoryServiceResult<Story> {
        let title = request.title.trim();
        if title.is_empty() {
            return Err(MissoryServiceError::validation("title is required"));
        }
        if title.chars().count() > 200 {
            return Err(MissoryServiceError::validation(
                "title must be at most 200 characters",
            ));
        }
        let scope = Self::scope(context);
        let participants = request.participant_ids.clone().unwrap_or_default();
        for participant in &participants {
            if self
                .store
                .get_person(&scope, *participant)
                .await
                .map_err(Self::map_store)?
                .is_none()
            {
                return Err(MissoryServiceError::not_found("person", *participant));
            }
        }
        let now = self.now_rfc3339();
        let story = Story {
            id: self.store.next_id().await.map_err(Self::map_store)?,
            title: title.to_string(),
            summary: None,
            summary_origin: None,
            participant_ids: participants,
            memory_ids: request.memory_ids.clone().unwrap_or_default(),
            started_at: request.started_at.clone(),
            ended_at: request.ended_at.clone(),
            location: request.location.clone(),
            created_at: now.clone(),
            updated_at: now,
        };
        self.store
            .insert_story(&scope, story.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(story)
    }

    async fn list_stories(
        &self,
        context: &MissoryRequestContext,
        query: ListStoriesQuery,
    ) -> MissoryServiceResult<MissoryPage<Story>> {
        let params = Self::page_params(query.page, query.page_size)?;
        let mut store_query = query;
        store_query.page = Some(params.page);
        store_query.page_size = Some(params.page_size);
        let (items, total) = self
            .store
            .list_stories(&Self::scope(context), store_query)
            .await
            .map_err(Self::map_store)?;
        Ok(Self::page(items, total, &params))
    }

    async fn get_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
    ) -> MissoryServiceResult<Story> {
        self.store
            .get_story(&Self::scope(context), story_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("story", story_id))
    }

    async fn update_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
        request: StoryUpsertRequest,
    ) -> MissoryServiceResult<Story> {
        let scope = Self::scope(context);
        let mut story = self
            .store
            .get_story(&scope, story_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("story", story_id))?;
        {
            let title = request.title.trim();
            if title.is_empty() {
                return Err(MissoryServiceError::validation("title is required"));
            }
            if !title.is_empty() {
                story.title = title.to_string();
            }
        }
        if let Some(participants) = request.participant_ids.clone() {
            for participant in &participants {
                if self
                    .store
                    .get_person(&scope, *participant)
                    .await
                    .map_err(Self::map_store)?
                    .is_none()
                {
                    return Err(MissoryServiceError::not_found("person", *participant));
                }
            }
            story.participant_ids = participants;
        }
        if let Some(memory_ids) = request.memory_ids.clone() {
            story.memory_ids = memory_ids;
        }
        if request.started_at.is_some() {
            story.started_at = request.started_at.clone();
        }
        if request.ended_at.is_some() {
            story.ended_at = request.ended_at.clone();
        }
        if request.location.is_some() {
            story.location = request.location.clone();
        }
        story.updated_at = self.now_rfc3339();
        self.store
            .update_story(&scope, story.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(story)
    }

    async fn delete_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
    ) -> MissoryServiceResult<()> {
        let scope = Self::scope(context);
        self.store
            .get_story(&scope, story_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("story", story_id))?;
        self.store
            .delete_story(&scope, story_id)
            .await
            .map_err(Self::map_store)?;
        Ok(())
    }

    async fn summarize_story(
        &self,
        context: &MissoryRequestContext,
        story_id: u64,
    ) -> MissoryServiceResult<Story> {
        let scope = Self::scope(context);
        let story = self
            .store
            .get_story(&scope, story_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("story", story_id))?;
        let mut memories = Vec::new();
        for memory_id in &story.memory_ids {
            if let Some(memory) = self
                .store
                .get_memory(&scope, *memory_id)
                .await
                .map_err(Self::map_store)?
            {
                memories.push(memory);
            }
        }
        let mut participant_names = Vec::with_capacity(story.participant_ids.len());
        for participant in &story.participant_ids {
            if let Some(person) = self
                .store
                .get_person(&scope, *participant)
                .await
                .map_err(Self::map_store)?
            {
                participant_names.push(person.display_name);
            }
        }
        let summary = assistant::compose_story_summary(
            &self.model,
            &story.title,
            &participant_names,
            story.location.as_deref(),
            story.started_at.as_deref(),
            &memories,
        )
        .await;
        let mut story = story;
        story.summary = Some(summary);
        story.summary_origin = Some(MemoryOrigin::Inference);
        story.updated_at = self.now_rfc3339();
        self.store
            .update_story(&scope, story.clone())
            .await
            .map_err(Self::map_store)?;
        Ok(story)
    }

    async fn list_reminders(
        &self,
        context: &MissoryRequestContext,
        query: ListRemindersQuery,
    ) -> MissoryServiceResult<MissoryPage<Reminder>> {
        let params = Self::page_params(query.page, query.page_size)?;
        let reminders = reminders::derive_reminders(
            &self.store,
            &Self::scope(context),
            self.now(),
            DEFAULT_LONG_UNCONTACTED_DAYS,
        )
        .await
        .map_err(Self::map_store)?;
        let visible =
            reminders::filter_by_state(&self.store, &Self::scope(context), reminders, self.now())
                .await
                .map_err(Self::map_store)?;
        let filtered = visible
            .into_iter()
            .filter(|reminder| {
                query
                    .reminder_type
                    .is_none_or(|kind| reminder.reminder_type == kind)
            })
            .collect::<Vec<_>>();
        let total = filtered.len() as i64;
        let offset = params.offset as usize;
        let items = filtered
            .into_iter()
            .skip(offset)
            .take(params.page_size as usize)
            .collect::<Vec<_>>();
        Ok(Self::page(items, total, &params))
    }

    async fn dismiss_reminder(
        &self,
        context: &MissoryRequestContext,
        reminder_id: &str,
    ) -> MissoryServiceResult<()> {
        reminders::assert_valid_reminder_key(reminder_id)?;
        self.store
            .put_reminder_state(
                &Self::scope(context),
                sdkwork_missory_spi::ReminderStateRecord {
                    reminder_id: reminder_id.to_string(),
                    status: sdkwork_missory_spi::ReminderStateStatus::Dismissed,
                    snoozed_until: None,
                    updated_at: self.now_rfc3339(),
                },
            )
            .await
            .map_err(Self::map_store)?;
        Ok(())
    }

    async fn snooze_reminder(
        &self,
        context: &MissoryRequestContext,
        reminder_id: &str,
        request: ReminderSnoozeRequest,
    ) -> MissoryServiceResult<()> {
        reminders::assert_valid_reminder_key(reminder_id)?;
        if !(1..=365).contains(&request.days) {
            return Err(MissoryServiceError::invalid_parameter(
                "days must be between 1 and 365",
            ));
        }
        let until = self.now() + time::Duration::days(request.days);
        self.store
            .put_reminder_state(
                &Self::scope(context),
                sdkwork_missory_spi::ReminderStateRecord {
                    reminder_id: reminder_id.to_string(),
                    status: sdkwork_missory_spi::ReminderStateStatus::Snoozed,
                    snoozed_until: Some(format_rfc3339(until)),
                    updated_at: self.now_rfc3339(),
                },
            )
            .await
            .map_err(Self::map_store)?;
        Ok(())
    }

    async fn get_home_digest(
        &self,
        context: &MissoryRequestContext,
    ) -> MissoryServiceResult<HomeDigest> {
        let scope = Self::scope(context);
        let reminders = reminders::derive_reminders(
            &self.store,
            &scope,
            self.now(),
            DEFAULT_LONG_UNCONTACTED_DAYS,
        )
        .await
        .map_err(Self::map_store)?;
        let today = reminders::filter_by_state(&self.store, &scope, reminders, self.now())
            .await
            .map_err(Self::map_store)?;
        let recent_memories = self
            .store
            .recent_memories(&scope, 5)
            .await
            .map_err(Self::map_store)?;
        let recent_persons = self
            .store
            .recent_persons(&scope, 5)
            .await
            .map_err(Self::map_store)?;
        Ok(HomeDigest {
            today_reminders: today.into_iter().take(5).collect(),
            recent_memories,
            recent_persons,
        })
    }

    async fn assistant_query(
        &self,
        context: &MissoryRequestContext,
        request: AssistantQueryRequest,
    ) -> MissoryServiceResult<AssistantAnswer> {
        let question = request.question.trim().to_string();
        if question.is_empty() {
            return Err(MissoryServiceError::validation("question is required"));
        }
        let scope = Self::scope(context);
        let (persons, _) = self
            .store
            .list_persons(
                &scope,
                ListPersonsQuery {
                    page: Some(1),
                    page_size: Some(200),
                    ..ListPersonsQuery::default()
                },
            )
            .await
            .map_err(Self::map_store)?;
        let memories = self
            .store
            .recent_memories(&scope, usize::MAX >> 4)
            .await
            .map_err(Self::map_store)?;
        let answer = assistant::answer_question(&question, &persons, &memories);
        Ok(answer)
    }

    async fn assistant_briefing(
        &self,
        context: &MissoryRequestContext,
        request: BriefingRequest,
    ) -> MissoryServiceResult<Briefing> {
        let detail = self.get_person(context, request.person_id).await?;
        let briefing = assistant::compose_briefing(&self.model, &detail, self.now_rfc3339()).await;
        Ok(briefing)
    }

    async fn assistant_message_draft(
        &self,
        context: &MissoryRequestContext,
        request: MessageDraftRequest,
    ) -> MissoryServiceResult<MessageDraft> {
        let detail = self.get_person(context, request.person_id).await?;
        let draft = assistant::compose_message_draft(&self.model, &detail, &request).await;
        Ok(draft)
    }

    async fn assistant_chat_summary(
        &self,
        context: &MissoryRequestContext,
        request: ChatSummaryRequest,
    ) -> MissoryServiceResult<ChatSummary> {
        let text = request.text.trim().to_string();
        if text.is_empty() {
            return Err(MissoryServiceError::validation("text is required"));
        }
        if text.chars().count() > 20_000 {
            return Err(MissoryServiceError::validation(
                "text must be at most 20000 characters",
            ));
        }
        let scope = Self::scope(context);
        let person = self
            .store
            .get_person(&scope, request.person_id)
            .await
            .map_err(Self::map_store)?
            .ok_or_else(|| MissoryServiceError::not_found("person", request.person_id))?;
        let candidates = extract::extract_candidates(person.id, &text, self.now());
        let mut saved_candidates = Vec::with_capacity(candidates.len());
        for mut candidate in candidates {
            candidate.id = self.store.next_id().await.map_err(Self::map_store)?;
            candidate.created_at = self.now_rfc3339();
            candidate.updated_at = self.now_rfc3339();
            let saved = self
                .store
                .insert_memory(&scope, candidate)
                .await
                .map_err(Self::map_store)?;
            saved_candidates.push(saved);
        }
        // Conversation import updates the relationship recency (PRD §21).
        let now = self.now_rfc3339();
        if let Some(mut edge) = self
            .store
            .get_relationship(&scope, person.id)
            .await
            .map_err(Self::map_store)?
        {
            edge.last_contacted_at = Some(now.clone());
            edge.updated_at = now.clone();
            self.store
                .upsert_relationship(&scope, edge)
                .await
                .map_err(Self::map_store)?;
        }
        let mut person_update = person.clone();
        person_update.last_contacted_at = Some(now.clone());
        person_update.updated_at = now;
        self.store
            .update_person(&scope, person_update)
            .await
            .map_err(Self::map_store)?;
        let summary = assistant::compose_chat_summary(&person.display_name, &saved_candidates);
        Ok(ChatSummary {
            person_id: person.id,
            summary,
            candidate_memories: saved_candidates,
        })
    }

    async fn create_data_export(
        &self,
        context: &MissoryRequestContext,
        _request: DataExportCreateRequest,
    ) -> MissoryServiceResult<DataExport> {
        let scope = Self::scope(context);
        let persons = self.collect_person_pages(&scope).await?;
        let mut relationships = Vec::with_capacity(persons.len());
        for person in &persons {
            if let Some(edge) = self
                .store
                .get_relationship(&scope, person.id)
                .await
                .map_err(Self::map_store)?
            {
                relationships.push(edge);
            }
        }
        let memories = self.collect_memory_pages(&scope).await?;
        let stories = self.collect_story_pages(&scope).await?;
        let derived = reminders::derive_reminders(
            &self.store,
            &scope,
            self.now(),
            DEFAULT_LONG_UNCONTACTED_DAYS,
        )
        .await
        .map_err(Self::map_store)?;
        let reminders = reminders::filter_by_state(&self.store, &scope, derived, self.now())
            .await
            .map_err(Self::map_store)?;
        Ok(DataExport {
            exported_at: self.now_rfc3339(),
            profile: self.get_my_profile(context).await?,
            persons,
            relationships,
            memories,
            stories,
            reminders,
        })
    }
}

/// Page size used when a service use case walks a whole collection.
const EXPORT_PAGE_SIZE: i64 = 200;
