//! `SqlxMissoryStore`: authoritative PostgreSQL implementation of the
//! `MissoryStore` SPI ports (tables prefixed `missory_`, subject-scoped by
//! `(tenant_id, user_id)` on every statement).
//!
//! Enum-typed DTO fields round-trip through their serde representations
//! (snake_case / kebab-case strings) stored in `VARCHAR` columns; JSON arrays
//! and maps use `JSONB`. Timestamps cross the wire as RFC 3339 strings and are
//! stored as `TIMESTAMPTZ`.

use std::sync::Arc;

use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_missory_contract::dto::{
    Memory, MemoryOrigin, MemoryStatus, MemoryType, MyProfile, Person, Relationship, SourceKind,
    Story,
};
use sdkwork_missory_contract::ports::{ListMemoriesQuery, ListPersonsQuery, ListStoriesQuery};
use sdkwork_missory_spi::{
    MissoryScope, MissoryStore, MissoryStoreError, MissoryStoreResult, ReminderStateRecord,
    ReminderStateStatus,
};
use sqlx::PgPool;
use sqlx::Row;

use crate::selects::{memory_select, person_select, relationship_select, story_select};
use time::OffsetDateTime;

/// PostgreSQL-backed missory store.
pub struct SqlxMissoryStore {
    pool: Arc<PgPool>,
    id_generator: SnowflakeIdGenerator,
}

fn store_error(context: &str, error: impl std::fmt::Display) -> MissoryStoreError {
    tracing::error!(context = %context, detail = %error, "missory sql store failure");
    MissoryStoreError::Storage(format!("{context}: {error}"))
}

fn not_found(what: &str) -> MissoryStoreError {
    MissoryStoreError::NotFound(what.to_owned())
}

fn format_rfc3339(value: OffsetDateTime) -> String {
    value
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_owned())
}

fn parse_rfc3339(value: &str) -> OffsetDateTime {
    OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
        .unwrap_or(OffsetDateTime::UNIX_EPOCH)
}

/// Enum round-trips via the serde representation (the exact wire strings).
mod enum_codec {
    use serde_json::Value;

    pub fn encode<T: serde::Serialize>(value: &T) -> String {
        match serde_json::to_value(value) {
            Ok(Value::String(text)) => text,
            _ => String::new(),
        }
    }

    pub fn decode<T: serde::de::DeserializeOwned>(text: &str) -> Option<T> {
        serde_json::from_value(Value::String(text.to_owned())).ok()
    }
}

use enum_codec::{decode, encode};

impl SqlxMissoryStore {
    /// Builds a store over an existing pool and allocated id generator.
    pub fn new(pool: Arc<PgPool>, id_generator: SnowflakeIdGenerator) -> Self {
        Self { pool, id_generator }
    }

    fn next_snowflake(&self) -> MissoryStoreResult<u64> {
        let raw = self
            .id_generator
            .generate()
            .map_err(|error| store_error("snowflake generate", error))?;
        u64::try_from(raw).map_err(|_| MissoryStoreError::storage("snowflake id was negative"))
    }
}

#[async_trait::async_trait]
impl MissoryStore for SqlxMissoryStore {
    async fn next_id(&self) -> MissoryStoreResult<u64> {
        self.next_snowflake()
    }

    async fn get_my_profile(&self, scope: &MissoryScope) -> MissoryStoreResult<Option<MyProfile>> {
        let row = sqlx::query_as::<
            _,
            (
                i64,
                String,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                serde_json::Value,
                serde_json::Value,
                serde_json::Value,
                Option<String>,
                Option<String>,
                OffsetDateTime,
                OffsetDateTime,
            ),
        >(
            "SELECT user_id, display_name, nickname, city, occupation, company, education, \
             interests, likes, dislikes, communication_style, bio, created_at, updated_at \
             FROM missory_my_profile WHERE tenant_id = $1 AND user_id = $2",
        )
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_my_profile", error))?;

        Ok(row.map(
            |(
                user_id,
                display_name,
                nickname,
                city,
                occupation,
                company,
                education,
                interests,
                likes,
                dislikes,
                communication_style,
                bio,
                created_at,
                updated_at,
            )| MyProfile {
                user_id: user_id as u64,
                display_name,
                nickname,
                city,
                occupation,
                company,
                education,
                interests: json_to_vec(interests),
                likes: json_to_vec(likes),
                dislikes: json_to_vec(dislikes),
                communication_style,
                bio,
                created_at: format_rfc3339(created_at),
                updated_at: format_rfc3339(updated_at),
            },
        ))
    }

    async fn put_my_profile(
        &self,
        scope: &MissoryScope,
        profile: MyProfile,
    ) -> MissoryStoreResult<MyProfile> {
        let now = OffsetDateTime::now_utc();
        sqlx::query(
            "INSERT INTO missory_my_profile
             (id, uuid, tenant_id, organization_id, user_id, display_name, nickname, city,
              occupation, company, education, interests, likes, dislikes, communication_style,
              bio, created_at, updated_at)
             VALUES ($1, $2, $3, 0, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $16)
             ON CONFLICT (tenant_id, user_id) DO UPDATE SET
               display_name = EXCLUDED.display_name, nickname = EXCLUDED.nickname,
               city = EXCLUDED.city, occupation = EXCLUDED.occupation, company = EXCLUDED.company,
               education = EXCLUDED.education, interests = EXCLUDED.interests,
               likes = EXCLUDED.likes, dislikes = EXCLUDED.dislikes,
               communication_style = EXCLUDED.communication_style, bio = EXCLUDED.bio,
               updated_at = EXCLUDED.updated_at",
        )
        .bind(self.next_snowflake()? as i64)
        .bind(uuid_for_id(scope.tenant_id, scope.user_id))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&profile.display_name)
        .bind(&profile.nickname)
        .bind(&profile.city)
        .bind(&profile.occupation)
        .bind(&profile.company)
        .bind(&profile.education)
        .bind(vec_to_json(&profile.interests))
        .bind(vec_to_json(&profile.likes))
        .bind(vec_to_json(&profile.dislikes))
        .bind(&profile.communication_style)
        .bind(&profile.bio)
        .bind(now)
        .execute(&*self.pool)
        .await
        .map_err(|error| store_error("put_my_profile", error))?;
        Ok(profile)
    }

    async fn insert_person(
        &self,
        scope: &MissoryScope,
        person: Person,
    ) -> MissoryStoreResult<Person> {
        let result = sqlx::query(
            "INSERT INTO missory_person
             (id, uuid, tenant_id, organization_id, user_id, display_name, aliases, gender,
              birthday, city, title, company, avatar_url, tags, interests, preferences, bio,
              contact_channels, notes, last_contacted_at, created_at, updated_at)
             VALUES ($1, $2, $3, 0, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
                     $17, $18, $19, $20, $20)",
        )
        .bind(person.id as i64)
        .bind(uuid_for_id(scope.tenant_id, person.id))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&person.display_name)
        .bind(vec_to_json(&person.aliases))
        .bind(&person.gender)
        .bind(&person.birthday)
        .bind(&person.city)
        .bind(&person.title)
        .bind(&person.company)
        .bind(&person.avatar_url)
        .bind(vec_to_json(&person.tags))
        .bind(vec_to_json(&person.interests))
        .bind(vec_to_json(&person.preferences))
        .bind(&person.bio)
        .bind(map_to_json(&person.contact_channels))
        .bind(&person.notes)
        .bind(person.last_contacted_at.as_deref().map(parse_rfc3339))
        .bind(parse_rfc3339(&person.created_at))
        .execute(&*self.pool)
        .await;
        match result {
            Ok(_) => Ok(person),
            Err(error) if is_unique_violation(&error) => Err(MissoryStoreError::Conflict(format!(
                "person {} already exists",
                person.id
            ))),
            Err(error) => Err(store_error("insert_person", error)),
        }
    }

    async fn update_person(
        &self,
        scope: &MissoryScope,
        person: Person,
    ) -> MissoryStoreResult<Person> {
        let result = sqlx::query(
            "UPDATE missory_person SET display_name = $3, aliases = $4, gender = $5, birthday = $6,
             city = $7, title = $8, company = $9, avatar_url = $10, tags = $11, interests = $12,
             preferences = $13, bio = $14, contact_channels = $15, notes = $16,
             last_contacted_at = $17, updated_at = $18
             WHERE tenant_id = $1 AND user_id = $2 AND id = $19",
        )
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&person.display_name)
        .bind(vec_to_json(&person.aliases))
        .bind(&person.gender)
        .bind(&person.birthday)
        .bind(&person.city)
        .bind(&person.title)
        .bind(&person.company)
        .bind(&person.avatar_url)
        .bind(vec_to_json(&person.tags))
        .bind(vec_to_json(&person.interests))
        .bind(vec_to_json(&person.preferences))
        .bind(&person.bio)
        .bind(map_to_json(&person.contact_channels))
        .bind(&person.notes)
        .bind(person.last_contacted_at.as_deref().map(parse_rfc3339))
        .bind(parse_rfc3339(&person.updated_at))
        .bind(person.id as i64)
        .execute(&*self.pool)
        .await
        .map_err(|error| store_error("update_person", error))?;
        if result.rows_affected() == 0 {
            return Err(not_found(&format!("person {}", person.id)));
        }
        Ok(person)
    }

    async fn delete_person(&self, scope: &MissoryScope, person_id: u64) -> MissoryStoreResult<()> {
        sqlx::query("DELETE FROM missory_person WHERE tenant_id = $1 AND user_id = $2 AND id = $3")
            .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
            .bind(i64::try_from(scope.user_id).unwrap_or(0))
            .bind(person_id as i64)
            .execute(&*self.pool)
            .await
            .map_err(|error| store_error("delete_person", error))?;
        Ok(())
    }

    async fn get_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<Option<Person>> {
        let row = sqlx::query(sqlx::AssertSqlSafe(
            person_select() + " WHERE tenant_id = $1 AND user_id = $2 AND id = $3",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(person_id as i64)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_person", error))?;
        Ok(row.as_ref().map(person_from_row))
    }

    async fn list_persons(
        &self,
        scope: &MissoryScope,
        query: ListPersonsQuery,
    ) -> MissoryStoreResult<(Vec<Person>, i64)> {
        let params = page_params(query.page, query.page_size);
        let pattern = like_pattern(query.q.as_deref());
        let rows = sqlx::query(sqlx::AssertSqlSafe(
            person_select()
                + " WHERE tenant_id = $1 AND user_id = $2
                   AND ($3 = '' OR display_name ILIKE $3 OR aliases::text ILIKE $3
                        OR tags::text ILIKE $3 OR COALESCE(title, '') ILIKE $3
                        OR COALESCE(company, '') ILIKE $3 OR COALESCE(notes, '') ILIKE $3)
                   ORDER BY updated_at DESC, id DESC
                   LIMIT $4 OFFSET $5",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&pattern)
        .bind(params.1)
        .bind((params.0 - 1).saturating_mul(params.1))
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("list_persons", error))?;
        let total: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(String::from(
            "SELECT COUNT(*) FROM missory_person WHERE tenant_id = $1 AND user_id = $2
             AND ($3 = '' OR display_name ILIKE $3 OR aliases::text ILIKE $3
                  OR tags::text ILIKE $3 OR COALESCE(title, '') ILIKE $3
                  OR COALESCE(company, '') ILIKE $3 OR COALESCE(notes, '') ILIKE $3)",
        )))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&pattern)
        .fetch_one(&*self.pool)
        .await
        .map_err(|error| store_error("list_persons total", error))?;
        Ok((rows.iter().map(person_from_row).collect(), total))
    }

    async fn recent_persons(
        &self,
        scope: &MissoryScope,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Person>> {
        let rows = sqlx::query(sqlx::AssertSqlSafe(
            person_select()
                + " WHERE tenant_id = $1 AND user_id = $2
                   ORDER BY COALESCE(last_contacted_at, updated_at) DESC, id DESC LIMIT $3",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(limit as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("recent_persons", error))?;
        Ok(rows.iter().map(person_from_row).collect())
    }

    async fn upsert_relationship(
        &self,
        scope: &MissoryScope,
        relationship: Relationship,
    ) -> MissoryStoreResult<Relationship> {
        let now = OffsetDateTime::now_utc();
        sqlx::query(
            "INSERT INTO missory_relationship
             (id, uuid, tenant_id, organization_id, user_id, person_id, relationship_types,
              started_at, last_contacted_at, description, importance, contact_cycle_days, notes,
              created_at, updated_at)
             VALUES ($1, $2, $3, 0, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $13)
             ON CONFLICT (tenant_id, user_id, person_id) DO UPDATE SET
               relationship_types = EXCLUDED.relationship_types,
               started_at = COALESCE(EXCLUDED.started_at, missory_relationship.started_at),
               last_contacted_at = COALESCE(EXCLUDED.last_contacted_at, missory_relationship.last_contacted_at),
               description = COALESCE(EXCLUDED.description, missory_relationship.description),
               importance = COALESCE(EXCLUDED.importance, missory_relationship.importance),
               contact_cycle_days = COALESCE(EXCLUDED.contact_cycle_days, missory_relationship.contact_cycle_days),
               notes = COALESCE(EXCLUDED.notes, missory_relationship.notes),
               updated_at = EXCLUDED.updated_at",
        )
        .bind(relationship.id as i64)
        .bind(uuid_for_id(scope.tenant_id, relationship.person_id))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(relationship.person_id as i64)
        .bind(serde_json::json!(relationship.relationship_types))
        .bind(relationship.started_at.as_deref().map(parse_rfc3339))
        .bind(relationship.last_contacted_at.as_deref().map(parse_rfc3339))
        .bind(&relationship.description)
        .bind(relationship.importance.map(|value| encode(&value)))
        .bind(relationship.contact_cycle_days)
        .bind(&relationship.notes)
        .bind(now)
        .execute(&*self.pool)
        .await
        .map_err(|error| store_error("upsert_relationship", error))?;
        Ok(relationship)
    }

    async fn delete_relationship(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<()> {
        sqlx::query("DELETE FROM missory_relationship WHERE tenant_id = $1 AND user_id = $2 AND person_id = $3")
            .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
            .bind(i64::try_from(scope.user_id).unwrap_or(0))
            .bind(person_id as i64)
            .execute(&*self.pool)
            .await
            .map_err(|error| store_error("delete_relationship", error))?;
        Ok(())
    }

    async fn get_relationship(
        &self,
        scope: &MissoryScope,
        person_id: u64,
    ) -> MissoryStoreResult<Option<Relationship>> {
        let row = sqlx::query(sqlx::AssertSqlSafe(
            relationship_select()
                + " WHERE r.tenant_id = $1 AND r.user_id = $2 AND r.person_id = $3",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(person_id as i64)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_relationship", error))?;
        Ok(row.as_ref().map(relationship_from_row))
    }

    async fn get_relationship_by_id(
        &self,
        scope: &MissoryScope,
        relationship_id: u64,
    ) -> MissoryStoreResult<Option<Relationship>> {
        let row = sqlx::query(sqlx::AssertSqlSafe(
            relationship_select() + " WHERE r.tenant_id = $1 AND r.user_id = $2 AND r.id = $3",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(relationship_id as i64)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_relationship_by_id", error))?;
        Ok(row.as_ref().map(relationship_from_row))
    }

    async fn insert_memory(
        &self,
        scope: &MissoryScope,
        memory: Memory,
    ) -> MissoryStoreResult<Memory> {
        let result = sqlx::query(
            "INSERT INTO missory_memory
             (id, uuid, tenant_id, organization_id, user_id, person_id, story_id, memory_type,
              title, content, origin, status, confidence, source_reason, source_kind, source_ref,
              importance, occurred_at, created_at, updated_at)
             VALUES ($1, $2, $3, 0, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
                     $17, $18, $18)",
        )
        .bind(memory.id as i64)
        .bind(uuid_for_id(scope.tenant_id, memory.id))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(memory.person_id as i64)
        .bind(memory.story_id.map(|value| value as i64))
        .bind(encode(&memory.memory_type))
        .bind(&memory.title)
        .bind(&memory.content)
        .bind(encode(&memory.origin))
        .bind(encode(&memory.status))
        .bind(memory.confidence)
        .bind(&memory.source_reason)
        .bind(encode(&memory.source_kind))
        .bind(&memory.source_ref)
        .bind(memory.importance.as_ref().map(encode))
        .bind(memory.occurred_at.as_deref().map(parse_rfc3339))
        .bind(parse_rfc3339(&memory.created_at))
        .execute(&*self.pool)
        .await;
        match result {
            Ok(_) => Ok(memory),
            Err(error) if is_unique_violation(&error) => Err(MissoryStoreError::Conflict(format!(
                "memory {} already exists",
                memory.id
            ))),
            Err(error) => Err(store_error("insert_memory", error)),
        }
    }

    async fn update_memory(
        &self,
        scope: &MissoryScope,
        memory: Memory,
    ) -> MissoryStoreResult<Memory> {
        let result = sqlx::query(
            "UPDATE missory_memory SET person_id = $3, story_id = $4, memory_type = $5,
             title = $6, content = $7, origin = $8, status = $9, confidence = $10,
             source_reason = $11, source_kind = $12, source_ref = $13, importance = $14,
             occurred_at = $15, updated_at = $16
             WHERE tenant_id = $1 AND user_id = $2 AND id = $17",
        )
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(memory.person_id as i64)
        .bind(memory.story_id.map(|value| value as i64))
        .bind(encode(&memory.memory_type))
        .bind(&memory.title)
        .bind(&memory.content)
        .bind(encode(&memory.origin))
        .bind(encode(&memory.status))
        .bind(memory.confidence)
        .bind(&memory.source_reason)
        .bind(encode(&memory.source_kind))
        .bind(&memory.source_ref)
        .bind(memory.importance.as_ref().map(encode))
        .bind(memory.occurred_at.as_deref().map(parse_rfc3339))
        .bind(parse_rfc3339(&memory.updated_at))
        .bind(memory.id as i64)
        .execute(&*self.pool)
        .await
        .map_err(|error| store_error("update_memory", error))?;
        if result.rows_affected() == 0 {
            return Err(not_found(&format!("memory {}", memory.id)));
        }
        Ok(memory)
    }

    async fn delete_memory(&self, scope: &MissoryScope, memory_id: u64) -> MissoryStoreResult<()> {
        sqlx::query("DELETE FROM missory_memory WHERE tenant_id = $1 AND user_id = $2 AND id = $3")
            .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
            .bind(i64::try_from(scope.user_id).unwrap_or(0))
            .bind(memory_id as i64)
            .execute(&*self.pool)
            .await
            .map_err(|error| store_error("delete_memory", error))?;
        Ok(())
    }

    async fn get_memory(
        &self,
        scope: &MissoryScope,
        memory_id: u64,
    ) -> MissoryStoreResult<Option<Memory>> {
        let row = sqlx::query(sqlx::AssertSqlSafe(
            memory_select() + " WHERE tenant_id = $1 AND user_id = $2 AND id = $3",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(memory_id as i64)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_memory", error))?;
        Ok(row.as_ref().map(memory_from_row))
    }

    async fn list_memories(
        &self,
        scope: &MissoryScope,
        query: ListMemoriesQuery,
    ) -> MissoryStoreResult<(Vec<Memory>, i64)> {
        let params = page_params(query.page, query.page_size);
        let pattern = like_pattern(query.q.as_deref());
        let rows = sqlx::query(sqlx::AssertSqlSafe(
            memory_select()
                + " WHERE tenant_id = $1 AND user_id = $2
                   AND ($3 = '' OR content ILIKE $3 OR COALESCE(title, '') ILIKE $3)
                   AND ($4::bigint IS NULL OR person_id = $4)
                   AND ($5 = '' OR memory_type = $5)
                   AND ($6 = '' OR status = $6)
                   AND ($7 = '' OR origin = $7)
                   ORDER BY COALESCE(occurred_at, created_at) DESC, id DESC
                   LIMIT $8 OFFSET $9",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&pattern)
        .bind(query.person_id.map(|value| value as i64))
        .bind(
            query
                .memory_type
                .map(|value| encode(&value))
                .unwrap_or_default(),
        )
        .bind(query.status.map(|value| encode(&value)).unwrap_or_default())
        .bind(query.origin.map(|value| encode(&value)).unwrap_or_default())
        .bind(params.1)
        .bind((params.0 - 1).saturating_mul(params.1))
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("list_memories", error))?;
        let total: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(String::from(
            "SELECT COUNT(*) FROM missory_memory WHERE tenant_id = $1 AND user_id = $2
             AND ($3 = '' OR content ILIKE $3 OR COALESCE(title, '') ILIKE $3)
             AND ($4::bigint IS NULL OR person_id = $4)
             AND ($5 = '' OR memory_type = $5) AND ($6 = '' OR status = $6)
             AND ($7 = '' OR origin = $7)",
        )))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&pattern)
        .bind(query.person_id.map(|value| value as i64))
        .bind(
            query
                .memory_type
                .map(|value| encode(&value))
                .unwrap_or_default(),
        )
        .bind(query.status.map(|value| encode(&value)).unwrap_or_default())
        .bind(query.origin.map(|value| encode(&value)).unwrap_or_default())
        .fetch_one(&*self.pool)
        .await
        .map_err(|error| store_error("list_memories total", error))?;
        Ok((rows.iter().map(memory_from_row).collect(), total))
    }

    async fn recent_memories(
        &self,
        scope: &MissoryScope,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Memory>> {
        let rows = sqlx::query(sqlx::AssertSqlSafe(memory_select() + " WHERE tenant_id = $1 AND user_id = $2 ORDER BY created_at DESC, id DESC LIMIT $3"))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(limit as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("recent_memories", error))?;
        Ok(rows.iter().map(memory_from_row).collect())
    }

    async fn memories_for_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Memory>> {
        let rows = sqlx::query(sqlx::AssertSqlSafe(
            memory_select()
                + " WHERE tenant_id = $1 AND user_id = $2 AND person_id = $3
                   ORDER BY COALESCE(occurred_at, created_at) DESC, id DESC LIMIT $4",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(person_id as i64)
        .bind(limit as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("memories_for_person", error))?;
        Ok(rows.iter().map(memory_from_row).collect())
    }

    async fn insert_story(&self, scope: &MissoryScope, story: Story) -> MissoryStoreResult<Story> {
        let result = sqlx::query(
            "INSERT INTO missory_story
             (id, uuid, tenant_id, organization_id, user_id, title, summary, summary_origin,
              participant_ids, memory_ids, started_at, ended_at, location, created_at, updated_at)
             VALUES ($1, $2, $3, 0, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $13)",
        )
        .bind(story.id as i64)
        .bind(uuid_for_id(scope.tenant_id, story.id))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&story.title)
        .bind(&story.summary)
        .bind(story.summary_origin.as_ref().map(encode))
        .bind(json_u64_vec(&story.participant_ids))
        .bind(json_u64_vec(&story.memory_ids))
        .bind(story.started_at.as_deref().map(parse_rfc3339))
        .bind(story.ended_at.as_deref().map(parse_rfc3339))
        .bind(&story.location)
        .bind(parse_rfc3339(&story.created_at))
        .execute(&*self.pool)
        .await;
        match result {
            Ok(_) => Ok(story),
            Err(error) if is_unique_violation(&error) => Err(MissoryStoreError::Conflict(format!(
                "story {} already exists",
                story.id
            ))),
            Err(error) => Err(store_error("insert_story", error)),
        }
    }

    async fn update_story(&self, scope: &MissoryScope, story: Story) -> MissoryStoreResult<Story> {
        let result = sqlx::query(
            "UPDATE missory_story SET title = $3, summary = $4, summary_origin = $5,
             participant_ids = $6, memory_ids = $7, started_at = $8, ended_at = $9,
             location = $10, updated_at = $11
             WHERE tenant_id = $1 AND user_id = $2 AND id = $12",
        )
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&story.title)
        .bind(&story.summary)
        .bind(story.summary_origin.as_ref().map(encode))
        .bind(json_u64_vec(&story.participant_ids))
        .bind(json_u64_vec(&story.memory_ids))
        .bind(story.started_at.as_deref().map(parse_rfc3339))
        .bind(story.ended_at.as_deref().map(parse_rfc3339))
        .bind(&story.location)
        .bind(parse_rfc3339(&story.updated_at))
        .bind(story.id as i64)
        .execute(&*self.pool)
        .await
        .map_err(|error| store_error("update_story", error))?;
        if result.rows_affected() == 0 {
            return Err(not_found(&format!("story {}", story.id)));
        }
        Ok(story)
    }

    async fn delete_story(&self, scope: &MissoryScope, story_id: u64) -> MissoryStoreResult<()> {
        sqlx::query("DELETE FROM missory_story WHERE tenant_id = $1 AND user_id = $2 AND id = $3")
            .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
            .bind(i64::try_from(scope.user_id).unwrap_or(0))
            .bind(story_id as i64)
            .execute(&*self.pool)
            .await
            .map_err(|error| store_error("delete_story", error))?;
        Ok(())
    }

    async fn get_story(
        &self,
        scope: &MissoryScope,
        story_id: u64,
    ) -> MissoryStoreResult<Option<Story>> {
        let row = sqlx::query(sqlx::AssertSqlSafe(
            story_select() + " WHERE tenant_id = $1 AND user_id = $2 AND id = $3",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(story_id as i64)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_story", error))?;
        Ok(row.as_ref().map(story_from_row))
    }

    async fn list_stories(
        &self,
        scope: &MissoryScope,
        query: ListStoriesQuery,
    ) -> MissoryStoreResult<(Vec<Story>, i64)> {
        let params = page_params(query.page, query.page_size);
        let pattern = like_pattern(query.q.as_deref());
        let rows = sqlx::query(sqlx::AssertSqlSafe(
            story_select()
                + " WHERE tenant_id = $1 AND user_id = $2
                   AND ($3 = '' OR title ILIKE $3 OR COALESCE(summary, '') ILIKE $3
                        OR COALESCE(location, '') ILIKE $3)
                   ORDER BY COALESCE(started_at, created_at) DESC, id DESC
                   LIMIT $4 OFFSET $5",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&pattern)
        .bind(params.1)
        .bind((params.0 - 1).saturating_mul(params.1))
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("list_stories", error))?;
        let total: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(String::from(
            "SELECT COUNT(*) FROM missory_story WHERE tenant_id = $1 AND user_id = $2
             AND ($3 = '' OR title ILIKE $3 OR COALESCE(summary, '') ILIKE $3
                  OR COALESCE(location, '') ILIKE $3)",
        )))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&pattern)
        .fetch_one(&*self.pool)
        .await
        .map_err(|error| store_error("list_stories total", error))?;
        Ok((rows.iter().map(story_from_row).collect(), total))
    }

    async fn stories_for_person(
        &self,
        scope: &MissoryScope,
        person_id: u64,
        limit: usize,
    ) -> MissoryStoreResult<Vec<Story>> {
        let rows = sqlx::query(sqlx::AssertSqlSafe(
            story_select()
                + " WHERE tenant_id = $1 AND user_id = $2 AND participant_ids @> $3::jsonb
                   ORDER BY COALESCE(started_at, created_at) DESC, id DESC LIMIT $4",
        ))
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(serde_json::json!([person_id]))
        .bind(limit as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|error| store_error("stories_for_person", error))?;
        Ok(rows.iter().map(story_from_row).collect())
    }

    async fn get_reminder_state(
        &self,
        scope: &MissoryScope,
        reminder_id: &str,
    ) -> MissoryStoreResult<Option<ReminderStateRecord>> {
        let row = sqlx::query_as::<_, (String, Option<OffsetDateTime>, OffsetDateTime)>(
            "SELECT status, snoozed_until, updated_at FROM missory_reminder_state
             WHERE tenant_id = $1 AND user_id = $2 AND reminder_id = $3",
        )
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(reminder_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| store_error("get_reminder_state", error))?;
        Ok(
            row.map(|(status, snoozed_until, updated_at)| ReminderStateRecord {
                reminder_id: reminder_id.to_owned(),
                status: if status == "snoozed" {
                    ReminderStateStatus::Snoozed
                } else {
                    ReminderStateStatus::Dismissed
                },
                snoozed_until: snoozed_until.map(format_rfc3339),
                updated_at: format_rfc3339(updated_at),
            }),
        )
    }

    async fn put_reminder_state(
        &self,
        scope: &MissoryScope,
        record: ReminderStateRecord,
    ) -> MissoryStoreResult<ReminderStateRecord> {
        let status = match record.status {
            ReminderStateStatus::Dismissed => "dismissed",
            ReminderStateStatus::Snoozed => "snoozed",
        };
        sqlx::query(
            "INSERT INTO missory_reminder_state
             (id, tenant_id, user_id, reminder_id, status, snoozed_until, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (tenant_id, user_id, reminder_id) DO UPDATE SET
               status = EXCLUDED.status, snoozed_until = EXCLUDED.snoozed_until,
               updated_at = EXCLUDED.updated_at",
        )
        .bind(self.next_snowflake()? as i64)
        .bind(i64::try_from(scope.tenant_id).unwrap_or(0))
        .bind(i64::try_from(scope.user_id).unwrap_or(0))
        .bind(&record.reminder_id)
        .bind(status)
        .bind(record.snoozed_until.as_deref().map(parse_rfc3339))
        .bind(parse_rfc3339(&record.updated_at))
        .execute(&*self.pool)
        .await
        .map_err(|error| store_error("put_reminder_state", error))?;
        Ok(record)
    }
}

// ---- select fragments + row mappers ----

fn person_from_row(row: &sqlx::postgres::PgRow) -> Person {
    Person {
        id: row_u64(row, "id"),
        display_name: row_string(row, "display_name"),
        aliases: row_json_vec(row, "aliases"),
        gender: row_opt_string(row, "gender"),
        birthday: row_opt_string(row, "birthday"),
        city: row_opt_string(row, "city"),
        title: row_opt_string(row, "title"),
        company: row_opt_string(row, "company"),
        avatar_url: row_opt_string(row, "avatar_url"),
        tags: row_json_vec(row, "tags"),
        interests: row_json_vec(row, "interests"),
        preferences: row_json_vec(row, "preferences"),
        bio: row_opt_string(row, "bio"),
        contact_channels: row_json_map(row, "contact_channels"),
        notes: row_opt_string(row, "notes"),
        last_contacted_at: row_opt_time(row, "last_contacted_at"),
        created_at: row_time(row, "created_at"),
        updated_at: row_time(row, "updated_at"),
    }
}

fn relationship_from_row(row: &sqlx::postgres::PgRow) -> Relationship {
    Relationship {
        id: row_u64(row, "id"),
        person_id: row_u64(row, "person_id"),
        relationship_types: decode_json_vec(row, "relationship_types"),
        started_at: row_opt_time(row, "started_at"),
        last_contacted_at: row_opt_time(row, "last_contacted_at"),
        description: row_opt_string(row, "description"),
        importance: row_opt_string(row, "importance").and_then(|value| decode(&value)),
        contact_cycle_days: row_opt_i64(row, "contact_cycle_days"),
        notes: row_opt_string(row, "notes"),
        created_at: row_time(row, "created_at"),
        updated_at: row_time(row, "updated_at"),
    }
}

fn memory_from_row(row: &sqlx::postgres::PgRow) -> Memory {
    Memory {
        id: row_u64(row, "id"),
        person_id: row_u64(row, "person_id"),
        story_id: row_opt_u64(row, "story_id"),
        memory_type: row_string(row, "memory_type")
            .pipe(|value| decode(&value))
            .unwrap_or(MemoryType::Semantic),
        title: row_opt_string(row, "title"),
        content: row_string(row, "content"),
        origin: row_string(row, "origin")
            .pipe(|value| decode(&value))
            .unwrap_or(MemoryOrigin::Fact),
        status: row_string(row, "status")
            .pipe(|value| decode(&value))
            .unwrap_or(MemoryStatus::Candidate),
        confidence: row_opt_f64(row, "confidence"),
        source_reason: row_opt_string(row, "source_reason"),
        source_kind: row_string(row, "source_kind")
            .pipe(|value| decode(&value))
            .unwrap_or(SourceKind::UserInput),
        source_ref: row_opt_string(row, "source_ref"),
        importance: row_opt_string(row, "importance").and_then(|value| decode(&value)),
        occurred_at: row_opt_time(row, "occurred_at"),
        created_at: row_time(row, "created_at"),
        updated_at: row_time(row, "updated_at"),
    }
}

fn story_from_row(row: &sqlx::postgres::PgRow) -> Story {
    Story {
        id: row_u64(row, "id"),
        title: row_string(row, "title"),
        summary: row_opt_string(row, "summary"),
        summary_origin: row_opt_string(row, "summary_origin").and_then(|value| decode(&value)),
        participant_ids: row_json_u64_vec(row, "participant_ids"),
        memory_ids: row_json_u64_vec(row, "memory_ids"),
        started_at: row_opt_time(row, "started_at"),
        ended_at: row_opt_time(row, "ended_at"),
        location: row_opt_string(row, "location"),
        created_at: row_time(row, "created_at"),
        updated_at: row_time(row, "updated_at"),
    }
}

// ---- small accessors (never fail: store reads degrade to defaults) ----

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}

fn row_u64(row: &sqlx::postgres::PgRow, column: &str) -> u64 {
    row.try_get::<i64, _>(column).map_or(0, |v| v as u64)
}

fn row_opt_u64(row: &sqlx::postgres::PgRow, column: &str) -> Option<u64> {
    row.try_get::<Option<i64>, _>(column)
        .ok()
        .flatten()
        .map(|v| v as u64)
}

fn row_opt_i64(row: &sqlx::postgres::PgRow, column: &str) -> Option<i64> {
    row.try_get::<Option<i64>, _>(column).ok().flatten()
}

fn row_opt_f64(row: &sqlx::postgres::PgRow, column: &str) -> Option<f64> {
    row.try_get::<Option<f64>, _>(column).ok().flatten()
}

fn row_string(row: &sqlx::postgres::PgRow, column: &str) -> String {
    row.try_get::<String, _>(column).unwrap_or_default()
}

fn row_opt_string(row: &sqlx::postgres::PgRow, column: &str) -> Option<String> {
    row.try_get::<Option<String>, _>(column).ok().flatten()
}

fn row_time(row: &sqlx::postgres::PgRow, column: &str) -> String {
    row.try_get::<OffsetDateTime, _>(column)
        .map_or_else(|_| "1970-01-01T00:00:00Z".to_owned(), format_rfc3339)
}

fn row_opt_time(row: &sqlx::postgres::PgRow, column: &str) -> Option<String> {
    row.try_get::<Option<OffsetDateTime>, _>(column)
        .ok()
        .flatten()
        .map(format_rfc3339)
}

fn row_json_u64_vec(row: &sqlx::postgres::PgRow, column: &str) -> Vec<u64> {
    row.try_get::<serde_json::Value, _>(column)
        .ok()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

fn row_json_vec(row: &sqlx::postgres::PgRow, column: &str) -> Vec<String> {
    row.try_get::<serde_json::Value, _>(column)
        .ok()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

fn decode_json_vec<T: serde::de::DeserializeOwned>(
    row: &sqlx::postgres::PgRow,
    column: &str,
) -> Vec<T> {
    row.try_get::<serde_json::Value, _>(column)
        .ok()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

fn row_json_map(
    row: &sqlx::postgres::PgRow,
    column: &str,
) -> std::collections::BTreeMap<String, String> {
    row.try_get::<serde_json::Value, _>(column)
        .ok()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

fn vec_to_json(values: &[String]) -> serde_json::Value {
    serde_json::json!(values)
}

fn json_u64_vec(values: &[u64]) -> serde_json::Value {
    serde_json::json!(values)
}

fn map_to_json(values: &std::collections::BTreeMap<String, String>) -> serde_json::Value {
    serde_json::json!(values)
}

fn json_to_vec(value: serde_json::Value) -> Vec<String> {
    serde_json::from_value(value).unwrap_or_default()
}

fn uuid_for_id(tenant_id: u64, id: u64) -> String {
    format!("missory-{tenant_id:x}-{id:016x}")
}

fn like_pattern(keyword: Option<&str>) -> String {
    match keyword.map(str::trim) {
        Some(text) if !text.is_empty() => format!("%{}%", text.to_lowercase().replace('%', "\\%")),
        _ => String::new(),
    }
}

fn page_params(page: Option<i64>, page_size: Option<i64>) -> (i64, i64) {
    let page = page.unwrap_or(1).clamp(1, 10_000);
    let page_size = page_size.unwrap_or(20).clamp(1, 200);
    (page, page_size)
}

fn is_unique_violation(error: &sqlx::Error) -> bool {
    match error.as_database_error() {
        Some(db) => db.code().is_some_and(|code| code == "23505"),
        None => false,
    }
}
