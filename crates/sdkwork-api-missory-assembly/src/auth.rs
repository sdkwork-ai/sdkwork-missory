//! IAM dual-token plane wiring for the Missory gateway (moved here from the
//! gateway so route/IAM composition stays in the owner assembly —
//! `API_ASSEMBLY_SPEC.md` gateway-integration rules).
//!
//! Identity model: requests carry `Authorization: Bearer <authToken>` +
//! `Access-Token: <accessToken>`; the `sdkwork-iam-web-adapter` resolver
//! verifies them against the IAM session database and
//! [`MissoryContextInjector`] projects the resolved principal into the same
//! [`MissoryRequestContext`] extension the route handlers consume. In
//! development the dev identity fallback runs INSIDE the framework pipeline so
//! anonymous requests reach handlers with the bypass identity; every other
//! environment is fail-closed (401 without valid IAM credentials).

use axum::extract::Request;
use axum::http::{header, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::Router;
use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};
use sdkwork_iam_web_adapter::{
    build_web_framework_layer, iam_web_request_context_resolver_from_database_pool_for_audiences,
    IamWebRequestContextResolver,
};
use sdkwork_missory_contract::context::MissoryRequestContext;
use sdkwork_routes_iam_app_api as iam_routes;

use crate::bootstrap::assemble_business_router_from_env;
use sdkwork_web_core::context_injection::DomainContextInjector;
use sdkwork_web_core::request_context::WebRequestContext;

/// Application audiences accepted from IAM-issued principals (`x-sdkwork`
/// application key and code; the production claim policy is enforced by the
/// resolver factory outside explicit development).
pub const IAM_AUDIENCES: [&str; 2] = ["sdkwork-missory", "missory"];

/// Default development identity used when the dev bypass is active.
pub const DEV_BYPASS_TENANT_ID: u64 = 1;
/// Default development owner user id.
pub const DEV_BYPASS_USER_ID: u64 = 1000;

/// True when the application database profile is configured (`SDKWORK_DATABASE_*`).
///
/// The missory store and the IAM session plane intentionally share one
/// PostgreSQL database (`iam_*` tables coexist with missory tables; the fleet
/// community gateway uses the same single-database shape), so the store
/// selection deciding Postgres-vs-memory also decides whether the IAM plane
/// can boot.
pub fn database_env_declared() -> bool {
    std::env::var("SDKWORK_DATABASE_URL").is_ok()
        || std::env::var("SDKWORK_DATABASE_ENGINE").is_ok()
}

/// IAM plane wired to the shared application database: the audience-bound
/// dual-token resolver plus the self-contained IAM app-api router (login /
/// registration / session surface, framework-wrapped by the IAM crate itself).
pub struct IamPlane {
    /// Audience-bound dual-token resolver for the missory app-api surface.
    pub resolver: IamWebRequestContextResolver,
    /// Self-contained IAM login/registration surface (framework-wrapped).
    pub router: Router,
}

/// Boots the IAM plane over the shared application database.
///
/// `build_sdkwork_iam_app_api_router_with_pool` bootstraps the IAM schema
/// idempotently, so a plain `db-migrate` against a fresh database is enough
/// before first start.
pub async fn build_iam_plane(pool: DatabasePool) -> Result<IamPlane, String> {
    let resolver = iam_web_request_context_resolver_from_database_pool_for_audiences(
        pool.clone(),
        &IAM_AUDIENCES,
    )
    .await?;
    let router = iam_routes::build_sdkwork_iam_app_api_router_with_pool(pool)
        .await
        .map_err(|error| format!("bootstrap iam app-api surface: {error}"))?;
    Ok(IamPlane { resolver, router })
}

/// Resolves the shared IAM plane pool; `None` keeps the memory-store dev
/// topology (development only) running without IAM.
pub async fn resolve_iam_plane_pool() -> Result<Option<DatabasePool>, String> {
    if !database_env_declared() {
        return Ok(None);
    }
    let config =
        DatabaseConfig::from_env("MISSORY").map_err(|error| format!("database config: {error}"))?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| format!("create iam plane database pool failed: {error}"))?;
    Ok(Some(pool))
}

/// Projects the framework-resolved principal into the domain request context.
///
/// Ids arrive as decimal strings from IAM; non-numeric principals (dev
/// fallback tokens) project no context, and the handler-level
/// `require_app_context` gate answers 401 — production principals are always
/// numeric ids.
pub struct MissoryContextInjector;

impl DomainContextInjector for MissoryContextInjector {
    fn inject(&self, request: &mut Request, context: &WebRequestContext) {
        let Some(principal) = context.principal.as_ref() else {
            return;
        };
        let (Ok(tenant_id), Ok(user_id)) = (
            principal.tenant_id().parse::<u64>(),
            principal.user_id().parse::<u64>(),
        ) else {
            return;
        };
        let organization_id = principal
            .organization_id()
            .and_then(|raw| raw.parse::<u64>().ok())
            .unwrap_or(0);
        request.extensions_mut().insert(MissoryRequestContext::new(
            tenant_id,
            organization_id,
            user_id,
        ));
    }
}

/// Development-only identity fallback: runs INSIDE the web-framework pipeline
/// so real dual-token requests resolve normally and anonymous development
/// requests still reach handlers with the bypass identity. In every
/// non-development environment the bypass is off (startup refuses it) and a
/// context-less request gets 401 — fail-closed.
pub async fn dev_identity_fallback(
    axum::extract::State(dev_bypass): axum::extract::State<bool>,
    mut request: Request,
    next: Next,
) -> Response {
    if request
        .extensions()
        .get::<MissoryRequestContext>()
        .is_none()
    {
        if !dev_bypass {
            return problem_unauthorized();
        }
        request.extensions_mut().insert(MissoryRequestContext::new(
            DEV_BYPASS_TENANT_ID,
            0,
            DEV_BYPASS_USER_ID,
        ));
    }
    next.run(request).await
}

fn problem_unauthorized() -> Response {
    let body = Json(serde_json::json!({
        "type": "about:blank",
        "title": "Authentication Required",
        "status": 401,
        "detail": "authentication required: send Authorization: Bearer <authToken> plus Access-Token (IAM dual token)",
        "code": 40101,
        "traceId": uuid_like_trace(),
    }));
    let mut response = (StatusCode::UNAUTHORIZED, body).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    response
}

/// Placeholder trace minting before the shared trace middleware lands.
fn uuid_like_trace() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|delta| delta.as_millis())
        .unwrap_or_default();
    format!("gw-{millis}")
}

/// Operator command: issues the deployment-provisioned credential-entry
/// bootstrap credential (dual tokens bound to a database session) for the
/// given tenant and application (`IAM_CREDENTIAL_ENTRY_SPEC.md`). The JSON
/// result feeds installers and credential-entry clients.
pub async fn issue_standalone_bootstrap_credential(
    tenant_id: &str,
    app_id: &str,
) -> Result<serde_json::Value, String> {
    let Some(pool) = sdkwork_database_sqlx::create_pool_from_env("MISSORY")
        .await
        .map_err(|error| format!("database pool: {error}"))?
    else {
        return Err(
            "no database configured: set SDKWORK_DATABASE_* to issue bootstrap credentials"
                .to_owned(),
        );
    };
    // The resolver factory auto-provisions the tenant application from
    // SDKWORK_APP_ROOT manifest discovery before issuance.
    let _resolver = sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_database_pool_for_audiences(
        pool.clone(),
        &IAM_AUDIENCES,
    )
    .await?;
    let sdkwork_database_sqlx::DatabasePool::Postgres(pg, _) = &pool else {
        return Err("bootstrap credential issuance requires the PostgreSQL engine".to_owned());
    };
    let issued = sdkwork_iam_web_adapter::issue_standalone_bootstrap_access_credential(
        pg, tenant_id, app_id, None,
    )
    .await?;
    Ok(serde_json::json!({
        "tenantId": issued.tenant_id,
        "appId": issued.app_id,
        "accessCredential": issued.access_credential,
        "authToken": issued.auth_token,
        "sessionId": issued.session_id,
        "expiresAt": issued.expires_at,
    }))
}

/// Public path prefixes for the same-origin web console
/// (`SDKWORK_MISSORY_STATIC_DIR`): manifest routes keep their declared auth,
/// while non-manifest paths (SPA deep links, assets, `/`) resolve public so
/// the gateway's static fallback serves them instead of an unclassified 401.
fn same_origin_console_public_prefixes() -> Vec<String> {
    match std::env::var("SDKWORK_MISSORY_STATIC_DIR") {
        Ok(dir) if !dir.trim().is_empty() => vec!["/".to_owned()],
        _ => Vec::new(),
    }
}

/// Composes the authenticated gateway router from environment configuration.
///
/// Order matters: the dev identity fallback runs INSIDE the IAM framework
/// pipeline (real dual-token requests resolve first; anonymous requests get
/// the dev identity in development or a 401 elsewhere), and the IAM login
/// surface merges beside the missory app-api plane when a database is
/// configured. Health routes are NOT included — the gateway mounts them via
/// the shared service router afterwards.
pub async fn compose_authenticated_router_from_env(dev_bypass: bool) -> Result<Router, String> {
    let iam_plane = match resolve_iam_plane_pool().await {
        Ok(Some(pool)) => match build_iam_plane(pool).await {
            Ok(plane) => {
                tracing::info!("iam dual-token plane ready (login surface mounted same-origin)");
                Some(plane)
            }
            Err(reason) => return Err(reason),
        },
        Ok(None) => None,
        Err(reason) => return Err(reason),
    };
    if iam_plane.is_none() && !dev_bypass {
        return Err(
            "refusing to compose: no IAM database configured and the development identity \
             bypass is off — every request would be rejected; set SDKWORK_DATABASE_URL or \
             enable SDKWORK_MISSORY_DEV_AUTH_BYPASS in development"
                .to_owned(),
        );
    }

    let business_router = assemble_business_router_from_env().await?;
    let resolver = match &iam_plane {
        Some(plane) => plane.resolver.clone(),
        None => IamWebRequestContextResolver::from_database_pool(None),
    };
    let missory_plane = sdkwork_web_axum::with_web_request_context(
        business_router.layer(axum::middleware::from_fn_with_state(
            dev_bypass,
            dev_identity_fallback,
        )),
        build_web_framework_layer(
            resolver,
            sdkwork_routes_missory_app_api::gateway_route_manifest(),
            same_origin_console_public_prefixes(),
        )
        .with_domain_injector(std::sync::Arc::new(MissoryContextInjector)),
    );

    let mut router = Router::new().merge(missory_plane);
    if let Some(plane) = iam_plane {
        router = router.merge(plane.router);
    }
    Ok(router)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_request() -> Request {
        Request::builder()
            .uri("/app/v3/api/missory/my_profile")
            .body(axum::body::Body::empty())
            .expect("request")
    }

    fn with_principal(
        principal: sdkwork_web_core::request_context::WebRequestPrincipal,
    ) -> WebRequestContext {
        WebRequestContext {
            request_id: sdkwork_web_core::request_identity::ServerRequestId(String::new()),
            api_surface: sdkwork_web_core::request_context::WebApiSurface::AppApi,
            auth_mode: sdkwork_web_core::request_context::WebAuthMode::DualToken,
            transport: sdkwork_web_core::request_context::WebTransportFacts {
                path: "/app/v3/api/missory/my_profile".to_owned(),
                method: "GET".to_owned(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                ingress_token_present: false,
                oauth_bearer_present: false,
                agent_token_present: false,
            },
            principal: Some(principal),
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: None,
            idempotency_key: None,
        }
    }

    #[test]
    fn given_numeric_principal_when_injected_then_domain_context_carries_ids() {
        let mut request = test_request();
        let principal = sdkwork_web_core::request_context::WebRequestPrincipal::builder()
            .tenant_id("7")
            .organization_id(Some("3".to_owned()))
            .user_id("1001")
            .build();
        let context = with_principal(principal);
        MissoryContextInjector.inject(&mut request, &context);
        let resolved = request
            .extensions()
            .get::<MissoryRequestContext>()
            .expect("context injected");
        assert_eq!(resolved.tenant_id, 7);
        assert_eq!(resolved.organization_id, 3);
        assert_eq!(resolved.user_id, 1001);
    }

    #[test]
    fn given_non_numeric_principal_when_injected_then_no_context_is_projected() {
        let mut request = test_request();
        let principal = sdkwork_web_core::request_context::WebRequestPrincipal::builder()
            .tenant_id("dev")
            .user_id("dev-user")
            .build();
        let context = with_principal(principal);
        MissoryContextInjector.inject(&mut request, &context);
        assert!(request
            .extensions()
            .get::<MissoryRequestContext>()
            .is_none());
    }
}
