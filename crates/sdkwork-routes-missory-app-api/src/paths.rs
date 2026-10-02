//! Route path constants (single source for router wiring and tests).

/// Owner profile resource.
pub const MY_PROFILE: &str = "/app/v3/api/missory/my_profile";
/// Persons collection.
pub const PERSONS: &str = "/app/v3/api/missory/persons";
/// One person resource.
pub const PERSON_BY_ID: &str = "/app/v3/api/missory/persons/{personId}";
/// Person relationship timeline.
pub const PERSON_TIMELINE: &str = "/app/v3/api/missory/persons/{personId}/timeline";
/// Person relationship edge.
pub const PERSON_RELATIONSHIPS: &str = "/app/v3/api/missory/persons/{personId}/relationships";
/// One relationship edge.
pub const PERSON_RELATIONSHIP_BY_ID: &str =
    "/app/v3/api/missory/persons/{personId}/relationships/{relationshipId}";
/// Memories collection.
pub const MEMORIES: &str = "/app/v3/api/missory/memories";
/// Memory extraction command (static segment wins over `{memoryId}`).
pub const MEMORIES_EXTRACT: &str = "/app/v3/api/missory/memories/extract";
/// One memory resource.
pub const MEMORY_BY_ID: &str = "/app/v3/api/missory/memories/{memoryId}";
/// Confirm command.
pub const MEMORY_CONFIRM: &str = "/app/v3/api/missory/memories/{memoryId}/confirm";
/// Reject command.
pub const MEMORY_REJECT: &str = "/app/v3/api/missory/memories/{memoryId}/reject";
/// Stories collection.
pub const STORIES: &str = "/app/v3/api/missory/stories";
/// One story resource.
pub const STORY_BY_ID: &str = "/app/v3/api/missory/stories/{storyId}";
/// Story summarize command.
pub const STORY_SUMMARY: &str = "/app/v3/api/missory/stories/{storyId}/summaries";
/// Reminders collection.
pub const REMINDERS: &str = "/app/v3/api/missory/reminders";
/// Reminder dismiss command.
pub const REMINDER_DISMISS: &str = "/app/v3/api/missory/reminders/{reminderId}/dismiss";
/// Reminder snooze command.
pub const REMINDER_SNOOZE: &str = "/app/v3/api/missory/reminders/{reminderId}/snooze";
/// Home digest.
pub const HOME_TODAY: &str = "/app/v3/api/missory/home/today";
/// Assistant query.
pub const ASSISTANT_QUERY: &str = "/app/v3/api/missory/assistant/query";
/// Assistant briefing.
pub const ASSISTANT_BRIEFINGS: &str = "/app/v3/api/missory/assistant/briefings";
/// Assistant message draft.
pub const ASSISTANT_MESSAGE_DRAFTS: &str = "/app/v3/api/missory/assistant/message_drafts";
/// Assistant chat summary.
pub const ASSISTANT_CHAT_SUMMARIES: &str = "/app/v3/api/missory/assistant/chat_summaries";
