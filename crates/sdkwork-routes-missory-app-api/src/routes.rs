//! Router wiring and thin handlers for the Missory app-api surface.
//!
//! Handlers parse the request, call the [`MissoryAppApi`] port once, and map the
//! result onto the standard envelope (`API_SPEC.md` §15). No business logic.

use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::routing::{delete, get, post};
use axum::Extension;
use axum::{Json, Router};
use sdkwork_missory_contract::context::MissoryRequestContext;
use sdkwork_missory_contract::dto::{
    AssistantQueryRequest, BriefingRequest, ChatSummaryRequest, DataExportCreateRequest,
    MemoryExtractRequest, MemoryUpsertRequest, MessageDraftRequest, MyProfileUpsertRequest,
    PersonUpsertRequest, RelationshipUpsertRequest, ReminderSnoozeRequest, StoryUpsertRequest,
};
use sdkwork_missory_contract::ports::MissoryAppApi;

use crate::error::ApiProblem;
use crate::paths;
use crate::query::{ListMemoriesParams, ListPersonsParams, ListRemindersParams, ListStoriesParams};
use crate::request_context::require_app_context;
use crate::response::{created_item, map_error, no_content, ok_command, ok_item, ok_page};

/// Shared router state.
#[derive(Clone)]
pub struct AppState {
    /// The contract port backing every handler.
    pub api: Arc<dyn MissoryAppApi>,
}

/// Builds the Missory app-api router (mounted under `/app/v3/api`).
pub fn build_router(api: Arc<dyn MissoryAppApi>) -> Router {
    let state = AppState { api };
    Router::new()
        .route(paths::MY_PROFILE, get(get_my_profile).put(put_my_profile))
        .route(paths::PERSONS, post(post_person).get(list_persons))
        .route(
            paths::PERSON_BY_ID,
            get(get_person).put(put_person).delete(delete_person),
        )
        .route(paths::PERSON_TIMELINE, get(get_person_timeline))
        .route(paths::PERSON_RELATIONSHIPS, post(post_relationship))
        .route(
            paths::PERSON_RELATIONSHIP_BY_ID,
            delete(delete_relationship),
        )
        .route(paths::MEMORIES, post(post_memory).get(list_memories))
        .route(paths::MEMORIES_EXTRACT, post(extract_memories))
        .route(
            paths::MEMORY_BY_ID,
            get(get_memory).put(put_memory).delete(delete_memory),
        )
        .route(paths::MEMORY_CONFIRM, post(confirm_memory))
        .route(paths::MEMORY_REJECT, post(reject_memory))
        .route(paths::STORIES, post(post_story).get(list_stories))
        .route(
            paths::STORY_BY_ID,
            get(get_story).put(put_story).delete(delete_story),
        )
        .route(paths::STORY_SUMMARY, post(summarize_story))
        .route(paths::REMINDERS, get(list_reminders))
        .route(paths::REMINDER_DISMISS, post(dismiss_reminder))
        .route(paths::REMINDER_SNOOZE, post(snooze_reminder))
        .route(paths::HOME_TODAY, get(get_home_today))
        .route(paths::ASSISTANT_QUERY, post(assistant_query))
        .route(paths::ASSISTANT_BRIEFINGS, post(assistant_briefing))
        .route(paths::ASSISTANT_MESSAGE_DRAFTS, post(assistant_draft))
        .route(
            paths::ASSISTANT_CHAT_SUMMARIES,
            post(assistant_chat_summary),
        )
        .route(paths::DATA_EXPORTS, post(post_data_export))
        .layer(Extension(state))
}

type HandlerResult = Result<axum::response::Response, ApiProblem>;

fn api_of(Extension(state): Extension<AppState>) -> Arc<dyn MissoryAppApi> {
    state.api
}

// ---- my profile ----

async fn get_my_profile(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_item(
        api_of(state)
            .get_my_profile(&context)
            .await
            .map_err(map_error)?,
    )
}

async fn put_my_profile(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<MyProfileUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    ok_item(
        api_of(state)
            .update_my_profile(&context, request)
            .await
            .map_err(map_error)?,
    )
}

// ---- persons ----

async fn post_person(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<PersonUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .create_person(&context, request)
            .await
            .map_err(map_error)?,
    )
}

async fn list_persons(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Query(params): Query<ListPersonsParams>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_page(
        api_of(state)
            .list_persons(&context, params.into())
            .await
            .map_err(map_error)?,
    )
}

async fn get_person(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(person_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_item(
        api_of(state)
            .get_person(&context, person_id)
            .await
            .map_err(map_error)?,
    )
}

async fn put_person(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(person_id): Path<u64>,
    body: Result<Json<PersonUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    ok_item(
        api_of(state)
            .update_person(&context, person_id, request)
            .await
            .map_err(map_error)?,
    )
}

async fn delete_person(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(person_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    api_of(state)
        .delete_person(&context, person_id)
        .await
        .map_err(map_error)?;
    no_content()
}

async fn get_person_timeline(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(person_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_page(
        api_of(state)
            .get_person_timeline(&context, person_id)
            .await
            .map_err(map_error)?,
    )
}

async fn post_relationship(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(person_id): Path<u64>,
    body: Result<Json<RelationshipUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .upsert_relationship(&context, person_id, request)
            .await
            .map_err(map_error)?,
    )
}

async fn delete_relationship(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path((person_id, relationship_id)): Path<(u64, u64)>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    api_of(state)
        .delete_relationship(&context, person_id, relationship_id)
        .await
        .map_err(map_error)?;
    no_content()
}

// ---- memories ----

async fn post_memory(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<MemoryUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .create_memory(&context, request)
            .await
            .map_err(map_error)?,
    )
}

async fn list_memories(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Query(params): Query<ListMemoriesParams>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_page(
        api_of(state)
            .list_memories(&context, params.into())
            .await
            .map_err(map_error)?,
    )
}

async fn get_memory(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(memory_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_item(
        api_of(state)
            .get_memory(&context, memory_id)
            .await
            .map_err(map_error)?,
    )
}

async fn put_memory(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(memory_id): Path<u64>,
    body: Result<Json<MemoryUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    ok_item(
        api_of(state)
            .update_memory(&context, memory_id, request)
            .await
            .map_err(map_error)?,
    )
}

async fn delete_memory(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(memory_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    api_of(state)
        .delete_memory(&context, memory_id)
        .await
        .map_err(map_error)?;
    no_content()
}

async fn confirm_memory(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(memory_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let memory = api_of(state)
        .confirm_memory(&context, memory_id)
        .await
        .map_err(map_error)?;
    ok_command(true, memory.id.to_string(), "confirmed".to_string())
}

async fn reject_memory(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(memory_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let memory = api_of(state)
        .reject_memory(&context, memory_id)
        .await
        .map_err(map_error)?;
    ok_command(true, memory.id.to_string(), "rejected".to_string())
}

async fn extract_memories(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<MemoryExtractRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    ok_page(
        api_of(state)
            .extract_memories(&context, request)
            .await
            .map_err(map_error)?,
    )
}

// ---- stories ----

async fn post_story(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<StoryUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .create_story(&context, request)
            .await
            .map_err(map_error)?,
    )
}

async fn list_stories(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Query(params): Query<ListStoriesParams>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_page(
        api_of(state)
            .list_stories(&context, params.into())
            .await
            .map_err(map_error)?,
    )
}

async fn get_story(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(story_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_item(
        api_of(state)
            .get_story(&context, story_id)
            .await
            .map_err(map_error)?,
    )
}

async fn put_story(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(story_id): Path<u64>,
    body: Result<Json<StoryUpsertRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    ok_item(
        api_of(state)
            .update_story(&context, story_id, request)
            .await
            .map_err(map_error)?,
    )
}

async fn delete_story(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(story_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    api_of(state)
        .delete_story(&context, story_id)
        .await
        .map_err(map_error)?;
    no_content()
}

async fn summarize_story(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(story_id): Path<u64>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    created_item(
        api_of(state)
            .summarize_story(&context, story_id)
            .await
            .map_err(map_error)?,
    )
}

// ---- reminders ----

async fn list_reminders(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Query(params): Query<ListRemindersParams>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_page(
        api_of(state)
            .list_reminders(&context, params.into())
            .await
            .map_err(map_error)?,
    )
}

async fn dismiss_reminder(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(reminder_id): Path<String>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    api_of(state)
        .dismiss_reminder(&context, &reminder_id)
        .await
        .map_err(map_error)?;
    ok_command(true, reminder_id, "dismissed".to_string())
}

async fn snooze_reminder(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    Path(reminder_id): Path<String>,
    body: Result<Json<ReminderSnoozeRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    api_of(state)
        .snooze_reminder(&context, &reminder_id, request)
        .await
        .map_err(map_error)?;
    ok_command(true, reminder_id, "snoozed".to_string())
}

// ---- home ----

async fn get_home_today(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    ok_item(
        api_of(state)
            .get_home_digest(&context)
            .await
            .map_err(map_error)?,
    )
}

// ---- assistant ----

async fn assistant_query(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<AssistantQueryRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    ok_item(
        api_of(state)
            .assistant_query(&context, request)
            .await
            .map_err(map_error)?,
    )
}

async fn assistant_briefing(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<BriefingRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .assistant_briefing(&context, request)
            .await
            .map_err(map_error)?,
    )
}

async fn assistant_draft(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<MessageDraftRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .assistant_message_draft(&context, request)
            .await
            .map_err(map_error)?,
    )
}

async fn assistant_chat_summary(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<ChatSummaryRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .assistant_chat_summary(&context, request)
            .await
            .map_err(map_error)?,
    )
}

// ---- privacy export ----

async fn post_data_export(
    state: Extension<AppState>,
    context: Option<Extension<MissoryRequestContext>>,
    body: Result<Json<DataExportCreateRequest>, axum::extract::rejection::JsonRejection>,
) -> HandlerResult {
    let context = require_app_context(context)?;
    let Json(request) =
        body.map_err(|rejection| ApiProblem::invalid_body(rejection.body_text()))?;
    created_item(
        api_of(state)
            .create_data_export(&context, request)
            .await
            .map_err(map_error)?,
    )
}
