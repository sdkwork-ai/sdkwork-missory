//! App-api route smoke tests over the real router with the in-memory store.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::Router;
use sdkwork_communication_missory_service::MissoryService;
use sdkwork_missory_contract::context::MissoryRequestContext;
use sdkwork_missory_plugin_store_memory::InMemoryMissoryStore;
use sdkwork_routes_missory_app_api::build_router_with_app_api;
use tower::ServiceExt;

const TENANT_HEADER: &str = "x-sdkwork-tenant-id";
const USER_HEADER: &str = "x-sdkwork-user-id";

/// Stand-in for the gateway context-injection middleware: the route crate
/// itself never parses identity headers (WEB_BACKEND_SPEC layering rule).
async fn inject_test_context(mut request: Request<Body>, next: Next) -> Response {
    if request
        .extensions()
        .get::<MissoryRequestContext>()
        .is_none()
    {
        let user_id = request
            .headers()
            .get(USER_HEADER)
            .and_then(|value| value.to_str().ok())
            .and_then(|raw| raw.parse::<u64>().ok());
        if let Some(user_id) = user_id {
            let tenant_id = request
                .headers()
                .get(TENANT_HEADER)
                .and_then(|value| value.to_str().ok())
                .and_then(|raw| raw.parse::<u64>().ok())
                .unwrap_or(1);
            request
                .extensions_mut()
                .insert(MissoryRequestContext::new(tenant_id, 0, user_id));
        }
    }
    next.run(request).await
}

fn test_router() -> Router {
    let store = InMemoryMissoryStore::new(1);
    let service = MissoryService::new(Arc::new(store), None);
    build_router_with_app_api(Arc::new(service)).layer(middleware::from_fn(inject_test_context))
}

fn authed_request(method: &str, uri: &str, body: Option<String>) -> Request<Body> {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(TENANT_HEADER, "1")
        .header(USER_HEADER, "1000");
    match body {
        Some(payload) => builder
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .expect("request"),
        None => builder.body(Body::empty()).expect("request"),
    }
}

async fn send_json(
    router: &Router,
    method: &str,
    uri: &str,
    body: Option<String>,
) -> (StatusCode, serde_json::Value) {
    let response = router
        .clone()
        .oneshot(authed_request(method, uri, body))
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let json = if bytes.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
    };
    (status, json)
}

#[tokio::test]
async fn given_missing_user_header_when_calling_then_unauthorized_problem_is_returned() {
    let router = test_router();
    let request = Request::builder()
        .method("GET")
        .uri("/app/v3/api/missory/my-profile")
        .body(Body::empty())
        .expect("request");
    let response = router.oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["code"], 40101);
    assert_eq!(json["status"], 401);
}

#[tokio::test]
async fn given_new_owner_when_getting_profile_then_default_profile_is_created() {
    let router = test_router();
    let (status, json) = send_json(&router, "GET", "/app/v3/api/missory/my-profile", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["code"], 0);
    assert!(json["traceId"].as_str().is_some());
    assert_eq!(json["data"]["item"]["userId"], "1000");
}

#[tokio::test]
async fn given_person_lifecycle_when_exercised_then_crud_semantics_hold() {
    let router = test_router();

    // create → 201 + item
    let (status, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(
            serde_json::json!({
                "displayName": "李明",
                "birthday": "05-20",
                "interests": ["摄影", "足球"],
                "relationshipTypes": ["classmate", "friend"]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{json}");
    let person_id = json["data"]["item"]["id"]
        .as_str()
        .expect("string id")
        .to_string();

    // retrieve detail → stats + relationship edge
    let (status, json) = send_json(
        &router,
        "GET",
        &format!("/app/v3/api/missory/persons/{person_id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["data"]["item"]["person"]["displayName"], "李明");
    assert_eq!(
        json["data"]["item"]["relationships"]["relationshipTypes"][0],
        "classmate"
    );

    // list with keyword filter → page envelope
    let (status, json) = send_json(
        &router,
        "GET",
        "/app/v3/api/missory/persons?page=1&page_size=20&q=%E6%9D%8E",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["data"]["items"].as_array().expect("items").len(), 1);
    assert_eq!(json["data"]["pageInfo"]["mode"], "offset");

    // delete → 204 without body
    let (status, body) = send_json(
        &router,
        "DELETE",
        &format!("/app/v3/api/missory/persons/{person_id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(body, serde_json::Value::Null);

    // retrieve again → 404 problem
    let (status, json) = send_json(
        &router,
        "GET",
        &format!("/app/v3/api/missory/persons/{person_id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["code"], 40401);
}

#[tokio::test]
async fn given_memory_extraction_when_confirmed_then_fact_inference_separation_holds() {
    let router = test_router();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "李明"}).to_string()),
    )
    .await;
    let person_id = json["data"]["item"]["id"].as_str().expect("id").to_string();

    // extract candidates from text: one stated fact + one hedged inference
    let (status, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/memories/extract",
        Some(
            serde_json::json!({
                "personId": person_id,
                "text": "李明喜欢摄影。听说他可能正在准备创业。我答应帮李明介绍一个客户。"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let items = json["data"]["items"].as_array().expect("items");
    assert!(
        items.len() >= 2,
        "expected fact+inference candidates: {json}"
    );
    let inference = items
        .iter()
        .find(|item| item["origin"] == "inference")
        .expect("an inference candidate");
    assert_eq!(inference["status"], "candidate");
    assert!(inference["confidence"].is_number());

    let inference_id = inference["id"].as_str().expect("id").to_string();

    // confirm command → accepted + promoted to user-confirmed fact
    let (status, json) = send_json(
        &router,
        "POST",
        &format!("/app/v3/api/missory/memories/{inference_id}/confirm"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["data"]["accepted"], true);

    let (_, json) = send_json(
        &router,
        "GET",
        &format!("/app/v3/api/missory/memories/{inference_id}"),
        None,
    )
    .await;
    assert_eq!(json["data"]["item"]["origin"], "fact");
    assert_eq!(json["data"]["item"]["status"], "confirmed");
    assert_eq!(json["data"]["item"]["sourceKind"], "user-input");
}

#[tokio::test]
async fn given_commitment_and_stale_contact_when_listing_reminders_then_due_items_are_derived() {
    let router = test_router();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "王强"}).to_string()),
    )
    .await;
    let person_id = json["data"]["item"]["id"].as_str().expect("id").to_string();

    // relationship with last contact far in the past and a 30-day cycle
    let (status, _) = send_json(
        &router,
        "POST",
        &format!("/app/v3/api/missory/persons/{person_id}/relationships"),
        Some(
            serde_json::json!({
                "relationshipTypes": ["colleague"],
                "lastContactedAt": "2026-01-01T00:00:00Z",
                "contactCycleDays": 30
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // commitment memory
    let (status, _) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/memories",
        Some(
            serde_json::json!({
                "personId": person_id,
                "type": "commitment",
                "content": "帮王强对接一位设计师"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, json) = send_json(&router, "GET", "/app/v3/api/missory/reminders", None).await;
    assert_eq!(status, StatusCode::OK);
    let items = json["data"]["items"].as_array().expect("items");
    let kinds: Vec<&str> = items
        .iter()
        .map(|item| item["type"].as_str().expect("type"))
        .collect();
    assert!(kinds.contains(&"long-uncontacted"), "{kinds:?}");
    assert!(kinds.contains(&"commitment"), "{kinds:?}");

    // dismiss the uncontacted reminder → filtered out
    let uncontacted = items
        .iter()
        .find(|item| item["type"] == "long-uncontacted")
        .expect("uncontacted reminder");
    let reminder_id = uncontacted["reminderId"].as_str().expect("key");
    let (status, json) = send_json(
        &router,
        "POST",
        &format!("/app/v3/api/missory/reminders/{reminder_id}/dismiss"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["data"]["accepted"], true);
    let (_, json) = send_json(&router, "GET", "/app/v3/api/missory/reminders", None).await;
    let kinds: Vec<&str> = json["data"]["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| item["type"].as_str().expect("type"))
        .collect();
    assert!(!kinds.contains(&"long-uncontacted"), "{kinds:?}");
}

#[tokio::test]
async fn given_assistant_queries_then_answers_cite_sources_without_fabrication() {
    let router = test_router();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "李明", "interests": ["摄影"]}).to_string()),
    )
    .await;
    let person_id = json["data"]["item"]["id"].as_str().expect("id").to_string();
    send_json(
        &router,
        "POST",
        "/app/v3/api/missory/memories",
        Some(
            serde_json::json!({
                "personId": person_id,
                "type": "episodic",
                "content": "2026年5月和李明一起去杭州旅行"
            })
            .to_string(),
        ),
    )
    .await;

    // person Q&A
    let (status, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/assistant/query",
        Some(serde_json::json!({"question": "李明是谁？"}).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert!(json["data"]["item"]["answer"]
        .as_str()
        .expect("answer")
        .contains("李明"));
    assert!(!json["data"]["item"]["citations"]
        .as_array()
        .expect("citations")
        .is_empty());

    // interest search
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/assistant/query",
        Some(serde_json::json!({"question": "谁喜欢摄影？"}).to_string()),
    )
    .await;
    assert!(json["data"]["item"]["answer"]
        .as_str()
        .expect("answer")
        .contains("李明"));

    // meeting briefing
    let (status, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/assistant/briefings",
        Some(serde_json::json!({"personId": person_id}).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{json}");
    assert!(json["data"]["item"]["briefing"]
        .as_str()
        .expect("briefing")
        .contains("见面简报"));

    // message draft — draft-only disclaimer present
    let (status, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/assistant/message-drafts",
        Some(
            serde_json::json!({"personId": person_id, "scenario": "birthday", "tone": "warm"})
                .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{json}");
    assert!(json["data"]["item"]["draft"].as_str().is_some());
    assert!(json["data"]["item"]["disclaimer"]
        .as_str()
        .expect("disclaimer")
        .contains("不会自动发送"));
}

#[tokio::test]
async fn given_story_with_memories_when_summarizing_then_ai_draft_summary_is_marked_inference() {
    let router = test_router();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "李明"}).to_string()),
    )
    .await;
    let person_id = json["data"]["item"]["id"].as_str().expect("id").to_string();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/memories",
        Some(
            serde_json::json!({
                "personId": person_id,
                "type": "episodic",
                "content": "一起爬了北高峰"
            })
            .to_string(),
        ),
    )
    .await;
    let memory_id = json["data"]["item"]["id"].as_str().expect("id").to_string();

    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/stories",
        Some(
            serde_json::json!({
                "title": "杭州之行",
                "participantIds": [person_id],
                "memoryIds": [memory_id],
                "startedAt": "2026-05-01T00:00:00Z",
                "location": "杭州"
            })
            .to_string(),
        ),
    )
    .await;
    let story_id = json["data"]["item"]["id"].as_str().expect("id").to_string();

    let (status, json) = send_json(
        &router,
        "POST",
        &format!("/app/v3/api/missory/stories/{story_id}/summaries"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{json}");
    assert!(json["data"]["item"]["summary"]
        .as_str()
        .expect("summary")
        .contains("杭州之行"));
    assert_eq!(json["data"]["item"]["summaryOrigin"], "inference");
}

#[tokio::test]
async fn given_home_when_requested_then_digest_sections_are_present() {
    let router = test_router();
    send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "李明"}).to_string()),
    )
    .await;
    let (status, json) = send_json(&router, "GET", "/app/v3/api/missory/home/today", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["data"]["item"]["todayReminders"].is_array());
    assert!(json["data"]["item"]["recentMemories"].is_array());
    assert!(json["data"]["item"]["recentPersons"].is_array());
}

#[tokio::test]
async fn given_out_of_range_page_size_when_listing_then_invalid_parameter_problem_is_returned() {
    let router = test_router();
    let (status, json) = send_json(
        &router,
        "GET",
        "/app/v3/api/missory/persons?page_size=500",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["code"], 40003);
}

#[tokio::test]
async fn given_timeline_when_requested_then_entries_cover_relationship_and_memories() {
    let router = test_router();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "李明"}).to_string()),
    )
    .await;
    let person_id = json["data"]["item"]["id"].as_str().expect("id").to_string();
    send_json(
        &router,
        "POST",
        &format!("/app/v3/api/missory/persons/{person_id}/relationships"),
        Some(
            serde_json::json!({
                "relationshipTypes": ["classmate"],
                "startedAt": "2014-09-01T00:00:00Z"
            })
            .to_string(),
        ),
    )
    .await;
    send_json(
        &router,
        "POST",
        "/app/v3/api/missory/memories",
        Some(
            serde_json::json!({
                "personId": person_id,
                "type": "episodic",
                "content": "毕业十周年聚会"
            })
            .to_string(),
        ),
    )
    .await;
    let (status, json) = send_json(
        &router,
        "GET",
        &format!("/app/v3/api/missory/persons/{person_id}/timeline"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let kinds: Vec<&str> = json["data"]["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| item["kind"].as_str().expect("kind"))
        .collect();
    assert!(kinds.contains(&"relationship_start"), "{kinds:?}");
    assert!(kinds.contains(&"memory"), "{kinds:?}");
}

#[tokio::test]
async fn given_chat_text_when_summarized_then_candidates_are_created_and_contact_refreshed() {
    let router = test_router();
    let (_, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/persons",
        Some(serde_json::json!({"displayName": "李明"}).to_string()),
    )
    .await;
    let person_id = json["data"]["item"]["id"].as_str().expect("id").to_string();

    let (status, json) = send_json(
        &router,
        "POST",
        "/app/v3/api/missory/assistant/chat-summaries",
        Some(
            serde_json::json!({
                "personId": person_id,
                "text": "李明说他在准备创业。"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{json}");
    assert!(json["data"]["item"]["summary"].as_str().is_some());
    assert!(!json["data"]["item"]["candidateMemories"]
        .as_array()
        .expect("candidates")
        .is_empty());

    // last-contact refreshed by the conversation import
    let (_, json) = send_json(
        &router,
        "GET",
        &format!("/app/v3/api/missory/persons/{person_id}"),
        None,
    )
    .await;
    assert!(json["data"]["item"]["person"]["lastContactedAt"].is_string());
}

#[tokio::test]
async fn given_request_context_contract_then_context_is_tenant_scoped() {
    let context = MissoryRequestContext::new(7, 0, 42);
    assert_eq!(context.tenant_id, 7);
    assert_eq!(context.user_id, 42);
}
