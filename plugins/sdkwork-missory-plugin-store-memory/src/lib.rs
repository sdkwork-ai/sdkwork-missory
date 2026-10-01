//! In-memory `MissoryStore` adapter for SDKWork Missory.
//!
//! Owner data is isolated by `(tenant_id, user_id)`; ids are minted as
//! snowflake-style `u64` values (millisecond timestamp, node id, sequence).
//! All operations are synchronous behind one map-level mutex and exposed
//! through the async port so the service layer stays storage-agnostic.

use std::collections::{BTreeMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::SystemTime;

use sdkwork_missory_contract::dto::{Memory, MyProfile, Person, Relationship, Story};
use sdkwork_missory_contract::ports::{ListMemoriesQuery, ListPersonsQuery, ListStoriesQuery};
use sdkwork_missory_spi::{
    MissoryScope, MissoryStore, MissoryStoreError, MissoryStoreResult, ReminderStateRecord,
};

/// Default long-uncontacted cycle applied when a relationship has no custom cycle.
pub const DEFAULT_CONTACT_CYCLE_DAYS: i64 = 30;

/// Per-owner isolated data.
#[derive(Default)]
struct OwnerData {
    my_profile: Option<MyProfile>,
    persons: BTreeMap<u64, Person>,
    relationships_by_person: BTreeMap<u64, Relationship>,
    memories: BTreeMap<u64, Memory>,
    stories: BTreeMap<u64, Story>,
    reminder_states: BTreeMap<String, ReminderStateRecord>,
}

/// In-memory Missory store.
pub struct InMemoryMissoryStore {
    node_id: u64,
    sequence: AtomicU64,
    owners: Mutex<BTreeMap<(u64, u64), OwnerData>>,
}

impl InMemoryMissoryStore {
    /// Creates a store with snowflake node id `node_id` (per-instance unique).
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id: node_id & 0x3FF,
            sequence: AtomicU64::new(0),
            owners: Mutex::new(BTreeMap::new()),
        }
    }

    /// Creates a store with node id from `SDKWORK_MISSORY_SNOWFLAKE_NODE_ID` (default 1).
    pub fn from_env() -> Self {
        let node_id = std::env::var("SDKWORK_MISSORY_SNOWFLAKE_NODE_ID")
            .ok()
            .and_then(|raw| raw.parse::<u64>().ok())
            .unwrap_or(1);
        Self::new(node_id)
    }

    fn owners_map(&self) -> MissoryStoreResult<MutexGuard<'_, BTreeMap<(u64, u64), OwnerData>>> {
        self.owners
            .lock()
            .map_err(|_| MissoryStoreError::storage("store lock poisoned"))
    }

    fn mint(&self) -> u64 {
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed) & 0xFFF;
        let millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|delta| delta.as_millis() as u64)
            .unwrap_or_default();
        (millis << 22) | (self.node_id << 12) | sequence
    }
}

/// Parses an RFC 3339 timestamp for ordering; falls back to epoch for bad input.
pub fn parse_instant(value: &str) -> time::OffsetDateTime {
    time::OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
        .unwrap_or(time::OffsetDateTime::UNIX_EPOCH)
}

fn person_matches(person: &Person, needle: &str) -> bool {
    let needle = needle.to_lowercase();
    person.display_name.to_lowercase().contains(&needle)
        || person
            .aliases
            .iter()
            .any(|alias| alias.to_lowercase().contains(&needle))
        || person
            .tags
            .iter()
            .any(|tag| tag.to_lowercase().contains(&needle))
        || person
            .title
            .as_deref()
            .is_some_and(|value| value.to_lowercase().contains(&needle))
        || person
            .company
            .as_deref()
            .is_some_and(|value| value.to_lowercase().contains(&needle))
        || person
            .notes
            .as_deref()
            .is_some_and(|value| value.to_lowercase().contains(&needle))
}

fn memory_matches(memory: &Memory, needle: &str) -> bool {
    let needle = needle.to_lowercase();
    memory.content.to_lowercase().contains(&needle)
        || memory
            .title
            .as_deref()
            .is_some_and(|value| value.to_lowercase().contains(&needle))
}

fn story_matches(story: &Story, needle: &str) -> bool {
    let needle = needle.to_lowercase();
    story.title.to_lowercase().contains(&needle)
        || story
            .summary
            .as_deref()
            .is_some_and(|value| value.to_lowercase().contains(&needle))
        || story
            .location
            .as_deref()
            .is_some_and(|value| value.to_lowercase().contains(&needle))
}

fn offset_window(page: Option<i64>, page_size: Option<i64>) -> (usize, usize) {
    let page = page.unwrap_or(1).max(1) as usize;
    let page_size = page_size.unwrap_or(20).clamp(1, 200) as usize;
    let offset = (page - 1) * page_size;
    (offset, page_size)
}

#[async_trait::async_trait]
impl MissoryStore for InMemoryMissoryStore {
    async fn next_id(&self) -> MissoryStoreResult<u64> {
        Ok(self.mint())
    }

    async fn get_my_profile(&self, scope: &MissoryScope) -> MissoryStoreResult<Option<MyProfile>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner.my_profile.clone())
    }

    async fn put_my_profile(
        &self,
        scope: &MissoryScope,
        profile: MyProfile,
    ) -> MissoryStoreResult<MyProfile> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.my_profile = Some(profile.clone());
        Ok(profile)
    }

    async fn insert_person(
        &self,
        scope: &MissoryScope,
        person: Person,
    ) -> MissoryStoreResult<Person> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        if owner.persons.contains_key(&person.id) {
            return Err(MissoryStoreError::conflict(format!(
                "person {} already exists",
                person.id
            )));
        }
        owner.persons.insert(person.id, person.clone());
        Ok(person)
    }

    async fn update_person(
        &self,
        scope: &MissoryScope,
        person: Person,
    ) -> MissoryStoreResult<Person> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.persons.insert(person.id, person.clone());
        Ok(person)
    }

    async fn delete_person(&self, scope: &MissoryScope, person_id: u64) -> MissoryStoreResult<()> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.persons.remove(&person_id);
        owner.relationships_by_person.remove(&person_id);
        let dead: HashSet<u64> = owner
            .memories
            .iter()
            .filter(|(_, memory)| memory.person_id == person_id)
            .map(|(id, _)| *id)
            .collect();
        for id in &dead {
            owner.memories.remove(id);
        }
        for story in owner.stories.values_mut() {
            story.participant_ids.retain(|id| *id != person_id);
            story.memory_ids.retain(|id| !dead.contains(id));
        }
        Ok(())
    }

    async fn get_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<Option<Person>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner.persons.get(&person_id).cloned())
    }

    async fn list_persons(
        &self,
        scope: &MissoryScope,
        query: ListPersonsQuery,
    ) -> MissoryStoreResult<(Vec<Person>, i64)> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut matched: Vec<Person> = owner
            .persons
            .values()
            .filter(|person| {
                query
                    .q
                    .as_deref()
                    .is_none_or(|needle| person_matches(person, needle))
            })
            .filter(|person| {
                query.relationship_type.is_none_or(|kind| {
                    owner
                        .relationships_by_person
                        .get(&person.id)
                        .is_some_and(|relationship| relationship.relationship_types.contains(&kind))
                })
            })
            .cloned()
            .collect();
        matched.sort_by(|left, right| {
            parse_instant(&right.updated_at)
                .cmp(&parse_instant(&left.updated_at))
                .then(right.id.cmp(&left.id))
        });
        let total = matched.len() as i64;
        let (offset, limit) = offset_window(query.page, query.page_size);
        Ok((
            matched.into_iter().skip(offset).take(limit).collect(),
            total,
        ))
    }

    async fn recent_persons(
        &self,
        scope: &MissoryScope,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Person>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut persons = owner.persons.values().cloned().collect::<Vec<_>>();
        persons.sort_by(|left, right| {
            let left_key = parse_instant(
                left.last_contacted_at
                    .as_deref()
                    .unwrap_or(left.updated_at.as_str()),
            );
            let right_key = parse_instant(
                right
                    .last_contacted_at
                    .as_deref()
                    .unwrap_or(right.updated_at.as_str()),
            );
            right_key.cmp(&left_key).then(right.id.cmp(&left.id))
        });
        persons.truncate(limit);
        Ok(persons)
    }

    async fn upsert_relationship(
        &self,
        scope: &MissoryScope,
        relationship: Relationship,
    ) -> MissoryStoreResult<Relationship> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner
            .relationships_by_person
            .insert(relationship.person_id, relationship.clone());
        Ok(relationship)
    }

    async fn delete_relationship(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<()> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.relationships_by_person.remove(&person_id);
        Ok(())
    }

    async fn get_relationship(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<Option<Relationship>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner.relationships_by_person.get(&person_id).cloned())
    }

    async fn get_relationship_by_id(
        &self,
        scope: &MissoryScope,
        relationship_id: u64,
    ) -> MissoryStoreResult<Option<Relationship>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner
            .relationships_by_person
            .values()
            .find(|relationship| relationship.id == relationship_id)
            .cloned())
    }

    async fn insert_memory(
        &self,
        scope: &MissoryScope,
        memory: Memory,
    ) -> MissoryStoreResult<Memory> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        if owner.memories.contains_key(&memory.id) {
            return Err(MissoryStoreError::conflict(format!(
                "memory {} already exists",
                memory.id
            )));
        }
        owner.memories.insert(memory.id, memory.clone());
        Ok(memory)
    }

    async fn update_memory(
        &self,
        scope: &MissoryScope,
        memory: Memory,
    ) -> MissoryStoreResult<Memory> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.memories.insert(memory.id, memory.clone());
        Ok(memory)
    }

    async fn delete_memory(&self, scope: &MissoryScope, memory_id: u64) -> MissoryStoreResult<()> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.memories.remove(&memory_id);
        Ok(())
    }

    async fn get_memory(
        &self,
        scope: &MissoryScope,
        memory_id: u64,
    ) -> MissoryStoreResult<Option<Memory>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner.memories.get(&memory_id).cloned())
    }

    async fn list_memories(
        &self,
        scope: &MissoryScope,
        query: ListMemoriesQuery,
    ) -> MissoryStoreResult<(Vec<Memory>, i64)> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut matched = owner
            .memories
            .values()
            .filter(|memory| {
                query
                    .q
                    .as_deref()
                    .is_none_or(|needle| memory_matches(memory, needle))
            })
            .filter(|memory| query.person_id.is_none_or(|id| memory.person_id == id))
            .filter(|memory| {
                query
                    .memory_type
                    .is_none_or(|kind| memory.memory_type == kind)
            })
            .filter(|memory| query.status.is_none_or(|status| memory.status == status))
            .filter(|memory| query.origin.is_none_or(|origin| memory.origin == origin))
            .cloned()
            .collect::<Vec<_>>();
        matched.sort_by(|left, right| {
            let left_key = parse_instant(
                left.occurred_at
                    .as_deref()
                    .unwrap_or(left.created_at.as_str()),
            );
            let right_key = parse_instant(
                right
                    .occurred_at
                    .as_deref()
                    .unwrap_or(right.created_at.as_str()),
            );
            right_key.cmp(&left_key).then(right.id.cmp(&left.id))
        });
        let total = matched.len() as i64;
        let (offset, limit) = offset_window(query.page, query.page_size);
        Ok((
            matched.into_iter().skip(offset).take(limit).collect(),
            total,
        ))
    }

    async fn recent_memories(
        &self,
        scope: &MissoryScope,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Memory>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut memories = owner.memories.values().cloned().collect::<Vec<_>>();
        memories.sort_by(|left, right| {
            parse_instant(&right.created_at)
                .cmp(&parse_instant(&left.created_at))
                .then(right.id.cmp(&left.id))
        });
        memories.truncate(limit);
        Ok(memories)
    }

    async fn memories_for_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Memory>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut memories = owner
            .memories
            .values()
            .filter(|memory| memory.person_id == person_id)
            .cloned()
            .collect::<Vec<_>>();
        memories.sort_by(|left, right| {
            let left_key = parse_instant(
                left.occurred_at
                    .as_deref()
                    .unwrap_or(left.created_at.as_str()),
            );
            let right_key = parse_instant(
                right
                    .occurred_at
                    .as_deref()
                    .unwrap_or(right.created_at.as_str()),
            );
            right_key.cmp(&left_key).then(right.id.cmp(&left.id))
        });
        memories.truncate(limit);
        Ok(memories)
    }

    async fn insert_story(&self, scope: &MissoryScope, story: Story) -> MissoryStoreResult<Story> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        if owner.stories.contains_key(&story.id) {
            return Err(MissoryStoreError::conflict(format!(
                "story {} already exists",
                story.id
            )));
        }
        owner.stories.insert(story.id, story.clone());
        Ok(story)
    }

    async fn update_story(&self, scope: &MissoryScope, story: Story) -> MissoryStoreResult<Story> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.stories.insert(story.id, story.clone());
        Ok(story)
    }

    async fn delete_story(&self, scope: &MissoryScope, story_id: u64) -> MissoryStoreResult<()> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner.stories.remove(&story_id);
        Ok(())
    }

    async fn get_story(
        &self,
        scope: &MissoryScope,
        story_id: u64,
    ) -> MissoryStoreResult<Option<Story>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner.stories.get(&story_id).cloned())
    }

    async fn list_stories(
        &self,
        scope: &MissoryScope,
        query: ListStoriesQuery,
    ) -> MissoryStoreResult<(Vec<Story>, i64)> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut matched = owner
            .stories
            .values()
            .filter(|story| {
                query
                    .q
                    .as_deref()
                    .is_none_or(|needle| story_matches(story, needle))
            })
            .cloned()
            .collect::<Vec<_>>();
        matched.sort_by(|left, right| {
            let left_key = parse_instant(left.started_at.as_deref().unwrap_or(&left.created_at));
            let right_key = parse_instant(right.started_at.as_deref().unwrap_or(&right.created_at));
            right_key.cmp(&left_key).then(right.id.cmp(&left.id))
        });
        let total = matched.len() as i64;
        let (offset, limit) = offset_window(query.page, query.page_size);
        Ok((
            matched.into_iter().skip(offset).take(limit).collect(),
            total,
        ))
    }

    async fn stories_for_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Story>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        let mut stories = owner
            .stories
            .values()
            .filter(|story| story.participant_ids.contains(&person_id))
            .cloned()
            .collect::<Vec<_>>();
        stories.sort_by(|left, right| {
            let left_key = parse_instant(left.started_at.as_deref().unwrap_or(&left.created_at));
            let right_key = parse_instant(right.started_at.as_deref().unwrap_or(&right.created_at));
            right_key.cmp(&left_key).then(right.id.cmp(&left.id))
        });
        stories.truncate(limit);
        Ok(stories)
    }

    async fn get_reminder_state(
        &self,
        scope: &MissoryScope,
        reminder_id: &str,
    ) -> MissoryStoreResult<Option<ReminderStateRecord>> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        Ok(owner.reminder_states.get(reminder_id).cloned())
    }

    async fn put_reminder_state(
        &self,
        scope: &MissoryScope,
        record: ReminderStateRecord,
    ) -> MissoryStoreResult<ReminderStateRecord> {
        let mut owners = self.owners_map()?;
        let owner = owners.entry((scope.tenant_id, scope.user_id)).or_default();
        owner
            .reminder_states
            .insert(record.reminder_id.clone(), record.clone());
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_person(id: u64, name: &str) -> Person {
        Person {
            id,
            display_name: name.to_string(),
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
            contact_channels: BTreeMap::new(),
            notes: None,
            last_contacted_at: None,
            created_at: "2026-10-01T00:00:00Z".to_string(),
            updated_at: "2026-10-01T00:00:00Z".to_string(),
        }
    }

    #[tokio::test]
    async fn given_two_scopes_when_inserting_same_person_id_then_scopes_are_isolated() {
        let store = InMemoryMissoryStore::new(1);
        store
            .insert_person(&MissoryScope::new(1, 1), sample_person(100, "李明"))
            .await
            .expect("insert into scope A");
        let other = store
            .get_person(&MissoryScope::new(1, 2), 100)
            .await
            .expect("read scope B");
        assert_eq!(other, None, "owner scopes must be isolated");
    }

    #[tokio::test]
    async fn given_existing_person_when_inserting_same_id_then_conflict_is_raised() {
        let store = InMemoryMissoryStore::new(1);
        let scope = MissoryScope::new(1, 2);
        store
            .insert_person(&scope, sample_person(7, "王芳"))
            .await
            .expect("first insert");
        let second = store.insert_person(&scope, sample_person(7, "王芳")).await;
        assert!(matches!(second, Err(MissoryStoreError::Conflict(_))));
    }

    #[tokio::test]
    async fn given_minted_ids_when_minting_repeatedly_then_ids_stay_unique() {
        let store = InMemoryMissoryStore::new(3);
        let first = store.next_id().await.expect("mint first");
        let second = store.next_id().await.expect("mint second");
        assert_ne!(first, second);
    }
}
