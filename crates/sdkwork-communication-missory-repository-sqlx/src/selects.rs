// Select fragments for the missory store. Kept as functions returning owned
// strings so every `sqlx::query(sqlx::AssertSqlSafe(...))` call site composes
// its final statement from these audited, compile-time-constant fragments
// (no user input ever reaches statement text — all values are bound).
#![allow(dead_code)]

/// Audited select fragment for `missory_person`.
pub fn person_select() -> String {
    "SELECT id, display_name, aliases, gender, birthday, city, title, company, avatar_url, tags, interests, preferences, bio, contact_channels, notes, last_contacted_at, created_at, updated_at FROM missory_person ".to_owned()
}

/// Audited select fragment for `missory_relationship`.
pub fn relationship_select() -> String {
    "SELECT r.id, r.person_id, r.relationship_types, r.started_at, r.last_contacted_at, r.description, r.importance, r.contact_cycle_days, r.notes, r.created_at, r.updated_at FROM missory_relationship r ".to_owned()
}

/// Audited select fragment for `missory_memory`.
pub fn memory_select() -> String {
    "SELECT id, person_id, story_id, memory_type, title, content, origin, status, confidence, source_reason, source_kind, source_ref, importance, occurred_at, created_at, updated_at FROM missory_memory ".to_owned()
}

/// Audited select fragment for `missory_story`.
pub fn story_select() -> String {
    "SELECT id, title, summary, summary_origin, participant_ids, memory_ids, started_at, ended_at, location, created_at, updated_at FROM missory_story ".to_owned()
}
