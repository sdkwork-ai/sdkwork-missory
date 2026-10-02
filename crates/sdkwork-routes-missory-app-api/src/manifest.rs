//! Runtime route manifest for the Missory app-api surface.
//!
//! Declares every owned operation with its framework authentication class so
//! the shared web-framework pipeline can resolve credentials, enforce
//! authorization, and apply rate limits (`API_ASSEMBLY_SPEC` §10; the JSON
//! route manifest under `sdks/_route-manifests/` stays generator-owned).
//!
//! Every Missory operation is owner-scoped personal data resolved from the
//! request context (`API_SPEC` §15.2), so routes are declared
//! [`RouteAuth::DualTokenOrAnonymous`]: the framework resolves the IAM
//! dual-token principal when credentials are present and falls back to an
//! anonymous context otherwise; the handler-level
//! `MissoryRequestContext` requirement still rejects unauthenticated calls
//! with 401.

use sdkwork_web_contract::{HttpMethod, HttpRoute, RouteAuth};
use sdkwork_web_core::HttpRouteManifest;

/// Missory operations (`/app/v3/api/missory/*`); order mirrors the OpenAPI
/// authority (`apis/app-api/communication/missory-app-api.openapi.json`).
const ROUTES: &[HttpRoute] = &[
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/my_profile",
        "missory",
        "myProfile.retrieve",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Put,
        "/app/v3/api/missory/my_profile",
        "missory",
        "myProfile.update",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/persons",
        "missory",
        "persons.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/persons",
        "missory",
        "persons.list",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/persons/{personId}",
        "missory",
        "persons.retrieve",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Put,
        "/app/v3/api/missory/persons/{personId}",
        "missory",
        "persons.update",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Delete,
        "/app/v3/api/missory/persons/{personId}",
        "missory",
        "persons.delete",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/persons/{personId}/timeline",
        "missory",
        "persons.timeline.list",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/persons/{personId}/relationships",
        "missory",
        "relationships.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Delete,
        "/app/v3/api/missory/persons/{personId}/relationships/{relationshipId}",
        "missory",
        "relationships.delete",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/memories",
        "missory",
        "memories.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/memories",
        "missory",
        "memories.list",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/memories/extract",
        "missory",
        "memories.extract",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/memories/{memoryId}",
        "missory",
        "memories.retrieve",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Put,
        "/app/v3/api/missory/memories/{memoryId}",
        "missory",
        "memories.update",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Delete,
        "/app/v3/api/missory/memories/{memoryId}",
        "missory",
        "memories.delete",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/memories/{memoryId}/confirm",
        "missory",
        "memories.confirm",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/memories/{memoryId}/reject",
        "missory",
        "memories.reject",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/stories",
        "missory",
        "stories.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/stories",
        "missory",
        "stories.list",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/stories/{storyId}",
        "missory",
        "stories.retrieve",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Put,
        "/app/v3/api/missory/stories/{storyId}",
        "missory",
        "stories.update",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Delete,
        "/app/v3/api/missory/stories/{storyId}",
        "missory",
        "stories.delete",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/stories/{storyId}/summaries",
        "missory",
        "stories.summaries.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/reminders",
        "missory",
        "reminders.list",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/reminders/{reminderId}/dismiss",
        "missory",
        "reminders.dismiss",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/reminders/{reminderId}/snooze",
        "missory",
        "reminders.snooze",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Get,
        "/app/v3/api/missory/home/today",
        "missory",
        "home.today.list",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/assistant/query",
        "missory",
        "assistant.query",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/assistant/briefings",
        "missory",
        "assistant.briefings.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/assistant/message_drafts",
        "missory",
        "assistant.messageDrafts.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
    HttpRoute::new(
        HttpMethod::Post,
        "/app/v3/api/missory/assistant/chat_summaries",
        "missory",
        "assistant.chatSummaries.create",
        RouteAuth::DualTokenOrAnonymous,
    ),
];

/// Build the owned runtime route manifest for the shared web-framework layer.
#[must_use]
pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_web_contract::HttpMethod as Method;

    #[test]
    fn given_manifest_when_matching_owned_routes_then_every_operation_is_declared() {
        let manifest = gateway_route_manifest();
        assert_eq!(manifest.routes().len(), 32);
        let sample = manifest.match_route("GET", "/app/v3/api/missory/persons/42/timeline");
        assert_eq!(
            sample.expect("timeline route").operation_id,
            "persons.timeline.list"
        );
    }

    #[test]
    fn given_manifest_when_classifying_auth_then_routes_are_dual_token_or_anonymous() {
        let manifest = gateway_route_manifest();
        for route in manifest.routes() {
            assert_eq!(
                route.auth,
                RouteAuth::DualTokenOrAnonymous,
                "route {} must stay DualTokenOrAnonymous",
                route.operation_id
            );
        }
    }

    #[test]
    fn given_unknown_path_when_matching_then_no_route_is_returned() {
        let manifest = gateway_route_manifest();
        assert!(manifest
            .match_route("GET", "/app/v3/api/missory/unknown")
            .is_none());
        assert!(manifest
            .match_route(Method::DELETE.as_str(), "/app/v3/api/missory/my_profile")
            .is_none());
    }
}
