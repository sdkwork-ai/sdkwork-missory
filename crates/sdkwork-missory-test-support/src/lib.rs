//! Shared test fixtures for SDKWork Missory.

use std::collections::BTreeMap;

use sdkwork_missory_contract::context::MissoryRequestContext;
use sdkwork_missory_contract::dto::{
    Importance, Memory, MemoryOrigin, MemoryStatus, MemoryType, Person, SourceKind,
};

/// Canonical test owner context (tenant 1, user 1000).
pub fn test_context() -> MissoryRequestContext {
    MissoryRequestContext::new(1, 0, 1000)
}

/// Builds a sample person with deterministic ids and timestamps.
pub fn sample_person(id: u64, display_name: &str, updated_at: &str) -> Person {
    Person {
        id,
        display_name: display_name.to_string(),
        aliases: vec![],
        gender: None,
        birthday: None,
        city: Some("北京".to_string()),
        title: Some("产品经理".to_string()),
        company: None,
        avatar_url: None,
        tags: vec![],
        interests: vec!["摄影".to_string()],
        preferences: vec![],
        bio: None,
        contact_channels: BTreeMap::new(),
        notes: None,
        last_contacted_at: None,
        created_at: "2014-09-01T00:00:00Z".to_string(),
        updated_at: updated_at.to_string(),
    }
}

/// Builds a sample confirmed semantic memory.
pub fn sample_memory(id: u64, person_id: u64, content: &str) -> Memory {
    Memory {
        id,
        person_id,
        story_id: None,
        memory_type: MemoryType::Semantic,
        title: None,
        content: content.to_string(),
        origin: MemoryOrigin::Fact,
        status: MemoryStatus::Confirmed,
        confidence: None,
        source_reason: None,
        source_kind: SourceKind::UserInput,
        source_ref: None,
        importance: Some(Importance::Normal),
        occurred_at: None,
        created_at: "2026-09-01T00:00:00Z".to_string(),
        updated_at: "2026-09-01T00:00:00Z".to_string(),
    }
}
