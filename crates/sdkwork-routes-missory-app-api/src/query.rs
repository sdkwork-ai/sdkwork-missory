//! Wire query parameter structs (camelCase, `API_SPEC.md` §14.1).
//!
//! Page and page_size are declared explicitly on every params struct:
//! `serde(flatten)` buffers values in a way that breaks `serde_urlencoded`
//! integer binding, so nesting is intentionally avoided here.

use sdkwork_missory_contract::dto::{MemoryOrigin, MemoryStatus, MemoryType, RelationshipType};
use sdkwork_missory_contract::ports::{
    ListMemoriesQuery, ListPersonsQuery, ListRemindersQuery, ListStoriesQuery,
};
use serde::Deserialize;

/// `GET /persons` query parameters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPersonsParams {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200); standard wire name is snake_case `page_size`.
    #[serde(rename = "page_size")]
    pub page_size: Option<i64>,
    /// Free-text keyword.
    pub q: Option<String>,
    /// Relationship type filter.
    pub relationship_type: Option<RelationshipType>,
}

impl From<ListPersonsParams> for ListPersonsQuery {
    fn from(params: ListPersonsParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
            q: params.q,
            relationship_type: params.relationship_type,
        }
    }
}

/// `GET /memories` query parameters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMemoriesParams {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200); standard wire name is snake_case `page_size`.
    #[serde(rename = "page_size")]
    pub page_size: Option<i64>,
    /// Free-text keyword.
    pub q: Option<String>,
    /// Person filter (wire: string id).
    pub person_id: Option<u64>,
    /// Memory type filter.
    #[serde(rename = "type")]
    pub memory_type: Option<MemoryType>,
    /// Status filter.
    pub status: Option<MemoryStatus>,
    /// Origin filter.
    pub origin: Option<MemoryOrigin>,
}

impl From<ListMemoriesParams> for ListMemoriesQuery {
    fn from(params: ListMemoriesParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
            q: params.q,
            person_id: params.person_id,
            memory_type: params.memory_type,
            status: params.status,
            origin: params.origin,
        }
    }
}

/// `GET /stories` query parameters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStoriesParams {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200); standard wire name is snake_case `page_size`.
    #[serde(rename = "page_size")]
    pub page_size: Option<i64>,
    /// Free-text keyword.
    pub q: Option<String>,
}

impl From<ListStoriesParams> for ListStoriesQuery {
    fn from(params: ListStoriesParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
            q: params.q,
        }
    }
}

/// `GET /reminders` query parameters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRemindersParams {
    /// 1-based page number.
    pub page: Option<i64>,
    /// Page size (1..=200); standard wire name is snake_case `page_size`.
    #[serde(rename = "page_size")]
    pub page_size: Option<i64>,
    /// Reminder type filter.
    #[serde(rename = "type")]
    pub reminder_type: Option<sdkwork_missory_contract::dto::ReminderType>,
}

impl From<ListRemindersParams> for ListRemindersQuery {
    fn from(params: ListRemindersParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
            reminder_type: params.reminder_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_person_params_when_parsed_then_camel_case_keys_bind() {
        let params: ListPersonsParams =
            serde_urlencoded::from_str("page=2&page_size=5&q=li&relationshipType=friend")
                .expect("parse");
        assert_eq!(params.page, Some(2));
        assert_eq!(params.page_size, Some(5));
        assert_eq!(params.relationship_type, Some(RelationshipType::Friend));
    }

    #[test]
    fn given_memory_params_when_parsed_then_type_key_binds_to_enum() {
        let params: ListMemoriesParams =
            serde_urlencoded::from_str("personId=42&type=preference&status=candidate")
                .expect("parse");
        assert_eq!(params.person_id, Some(42));
        assert_eq!(params.memory_type, Some(MemoryType::Preference));
        assert_eq!(params.status, Some(MemoryStatus::Candidate));
    }

    #[test]
    fn given_reminder_params_when_parsed_then_kebab_type_binds() {
        let params: ListRemindersParams =
            serde_urlencoded::from_str("type=long-uncontacted&page=1").expect("parse");
        assert_eq!(
            params.reminder_type,
            Some(sdkwork_missory_contract::dto::ReminderType::LongUncontacted)
        );
    }
}
