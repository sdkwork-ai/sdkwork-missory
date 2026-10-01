//! Standalone gateway for the sdkwork-missory application HTTP plane.
//!
//! Process concerns only: fail-closed environment validation, tracing, dev
//! request-context injection, CORS, binding, and graceful shutdown.

use std::sync::Arc;
use std::time::Duration;

use axum::extract::Request;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_missory_contract::context::MissoryRequestContext;

/// Environment variables governing startup.
const ENVIRONMENT_KEY: &str = "SDKWORK_MISSORY_ENVIRONMENT";
const RUNTIME_TARGET_KEY: &str = "SDKWORK_MISSORY_RUNTIME_TARGET";
const BIND_KEY: &str = "SDKWORK_MISSORY_APPLICATION_PUBLIC_INGRESS_BIND";
/// Dev-only identity bypass (`CORS_SPEC.md`/ENVIRONMENT_SPEC: development topologies only).
const DEV_AUTH_BYPASS_KEY: &str = "SDKWORK_MISSORY_DEV_AUTH_BYPASS";
/// Shared browser-origin allowlist (`CORS_SPEC.md` section 5; app-scoped keys are retired).
const CORS_ORIGINS_KEY: &str = "SDKWORK_CORS_ALLOWED_ORIGINS";
/// Header carrying the resolved tenant id (pre-IAM local topology).
const TENANT_HEADER: &str = "x-sdkwork-tenant-id";
/// Header carrying the resolved user id (pre-IAM local topology).
const USER_HEADER: &str = "x-sdkwork-user-id";

/// Default development identity used when the dev bypass is active.
pub const DEV_BYPASS_TENANT_ID: u64 = 1;
pub const DEV_BYPASS_USER_ID: u64 = 1000;

/// Allowed environments (`CONFIG_SPEC.md` §2).
const ALLOWED_ENVIRONMENTS: [&str; 5] = ["development", "test", "staging", "demo", "production"];

fn default_bind() -> &'static str {
    "127.0.0.1:8460"
}

/// Resolves and validates the startup environment; fails closed when absent.
fn resolve_environment() -> Result<String, String> {
    let runtime_target = std::env::var(RUNTIME_TARGET_KEY).unwrap_or_default();
    if runtime_target == "test-runner" {
        return Ok("test".to_string());
    }
    match std::env::var(ENVIRONMENT_KEY) {
        Ok(value) if ALLOWED_ENVIRONMENTS.contains(&value.as_str()) => Ok(value),
        Ok(other) => Err(format!(
            "{ENVIRONMENT_KEY} must be one of development|test|staging|demo|production (got {other})"
        )),
        Err(_) => Err(format!(
            "refusing to start: {ENVIRONMENT_KEY} is required (development|test|staging|demo|production)"
        )),
    }
}

/// Resolves the dev identity bypass; fail-closed outside development.
///
/// The bypass exists so the standalone development topology (browser dev
/// servers, Flutter local run, mini-program devtools) can reach the app-api
/// before the IAM dual-token adapter lands. It MUST be false in every
/// non-development environment: enabling it elsewhere refuses startup.
fn resolve_dev_bypass(environment: &str) -> Result<bool, String> {
    let raw = std::env::var(DEV_AUTH_BYPASS_KEY).unwrap_or_default();
    let enabled = matches!(raw.as_str(), "true" | "1" | "yes");
    if enabled && environment != "development" {
        return Err(format!(
            "refusing to start: {DEV_AUTH_BYPASS_KEY}=true is only allowed with {ENVIRONMENT_KEY}=development (got {environment})"
        ));
    }
    Ok(enabled)
}

/// Parses the shared CORS allowlist into exact origins.
///
/// Values are comma- or semicolon-separated origin strings; entries without a
/// scheme that look like hosts are left verbatim (the allowlist only ever
/// matches exact `Origin` header values).
fn parse_cors_origins(raw: &str) -> Vec<String> {
    raw.split([',', ';'])
        .map(|origin| origin.trim().to_string())
        .filter(|origin| !origin.is_empty())
        .collect()
}

/// CORS allowlist middleware (`CORS_SPEC.md`): preflight answers 204 with the
/// exact allowlisted origin reflected; simple responses carry the allowlist
/// headers. Non-allowlisted origins get no CORS headers and browsers block.
async fn cors_allowlist(
    origins: Arc<Vec<String>>,
    request: Request,
    next: Next,
) -> Response {
    let request_origin = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let mut response = next.run(request).await;
    let Some(origin) = request_origin else {
        return response;
    };
    if !origins.iter().any(|allowed| allowed == &origin) {
        return response;
    }
    let headers = response.headers_mut();
    if let Ok(value) = HeaderValue::from_str(&origin) {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, value);
    }
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    headers.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("x-sdkwork-trace-id"),
    );
    headers.insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

fn preflight_response(origin: &str) -> Response {
    let mut response = StatusCode::NO_CONTENT.into_response();
    let headers = response.headers_mut();
    if let Ok(value) = HeaderValue::from_str(origin) {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, value);
    }
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET,POST,PUT,PATCH,DELETE,OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static(
            "authorization,content-type,access-token,x-sdkwork-tenant-id,x-sdkwork-user-id",
        ),
    );
    headers.insert(
        header::ACCESS_CONTROL_MAX_AGE,
        HeaderValue::from_static("600"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    headers.insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

fn parse_header_id(request: &Request, name: &str) -> Option<u64> {
    request
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|raw| raw.parse::<u64>().ok())
}

/// Context-injection middleware: explicit headers (pre-IAM local topology)
/// first, then the development bypass default, then a 401 problem. The bypass
/// is gated at startup by [`resolve_dev_bypass`].
async fn context_injection(
    axum::extract::State(dev_bypass): axum::extract::State<bool>,
    mut request: Request,
    next: Next,
) -> Response {
    if request.extensions().get::<MissoryRequestContext>().is_none() {
        let header_user = parse_header_id(&request, USER_HEADER);
        let context = match header_user {
            Some(user_id) => {
                let tenant_id = parse_header_id(&request, TENANT_HEADER).unwrap_or(1);
                Some(MissoryRequestContext::new(tenant_id, 0, user_id))
            }
            None if dev_bypass => Some(MissoryRequestContext::new(
                DEV_BYPASS_TENANT_ID,
                0,
                DEV_BYPASS_USER_ID,
            )),
            None => None,
        };
        match context {
            Some(context) => {
                request.extensions_mut().insert(context);
            }
            None => return problem_unauthorized(),
        }
    }
    next.run(request).await
}

fn problem_unauthorized() -> Response {
    let body = Json(serde_json::json!({
        "type": "about:blank",
        "title": "Authentication Required",
        "status": 401,
        "detail": format!("missing {USER_HEADER} header (pre-IAM dev topology)"),
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

#[cfg(unix)]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install ctrl_c handler");
    };
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received; draining connections");
    tokio::time::sleep(Duration::from_millis(50)).await;
}

#[cfg(not(unix))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("install ctrl_c handler");
    tracing::info!("shutdown signal received; draining connections");
    tokio::time::sleep(Duration::from_millis(50)).await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    let environment = match resolve_environment() {
        Ok(value) => value,
        Err(reason) => {
            tracing::error!("{reason}");
            std::process::exit(2);
        }
    };
    let dev_bypass = match resolve_dev_bypass(&environment) {
        Ok(value) => value,
        Err(reason) => {
            tracing::error!("{reason}");
            std::process::exit(2);
        }
    };
    let cors_origins: Arc<Vec<String>> = Arc::new(parse_cors_origins(
        &std::env::var(CORS_ORIGINS_KEY).unwrap_or_default(),
    ));
    tracing::info!(
        environment = %environment,
        dev_bypass,
        cors_origins = cors_origins.len(),
        "starting sdkwork-missory standalone gateway"
    );

    let business_router = sdkwork_api_missory_assembly::assemble_business_router_from_env()
        .expect("assemble api router")
        .layer(middleware::from_fn_with_state(dev_bypass, context_injection))
        .layer(middleware::from_fn_with_state(
            cors_origins.clone(),
            |axum::extract::State(state): axum::extract::State<Arc<Vec<String>>>,
             request: Request,
             next: Next| async move {
                if request.method() == Method::OPTIONS {
                    let origin = request
                        .headers()
                        .get(header::ORIGIN)
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);
                    if let Some(origin) =
                        origin.filter(|value| state.iter().any(|allowed| allowed == value))
                    {
                        return preflight_response(&origin);
                    }
                }
                cors_allowlist(state, request, next).await
            },
        ));
    let router = sdkwork_web_bootstrap::service_router(
        business_router,
        sdkwork_web_bootstrap::ServiceRouterConfig::default().with_always_ready(),
    );

    let bind = std::env::var(BIND_KEY).unwrap_or_else(|_| default_bind().to_string());
    let listener = match tokio::net::TcpListener::bind(&bind).await {
        Ok(listener) => listener,
        Err(error) => {
            tracing::error!("failed to bind {bind}: {error}");
            std::process::exit(2);
        }
    };
    tracing::info!(bind = %bind, "missory app-api listening");
    if let Err(error) = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        tracing::error!("server error: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_bypass_outside_development_then_startup_refuses() {
        std::env::set_var(DEV_AUTH_BYPASS_KEY, "true");
        let result = resolve_dev_bypass("production");
        std::env::remove_var(DEV_AUTH_BYPASS_KEY);
        assert!(result.is_err(), "bypass must fail closed outside development");
    }

    #[test]
    fn given_bypass_in_development_then_startup_allows() {
        std::env::set_var(DEV_AUTH_BYPASS_KEY, "true");
        let result = resolve_dev_bypass("development");
        std::env::remove_var(DEV_AUTH_BYPASS_KEY);
        assert!(result.expect("allowed"));
    }

    #[test]
    fn given_unset_bypass_then_disabled() {
        std::env::remove_var(DEV_AUTH_BYPASS_KEY);
        assert!(!resolve_dev_bypass("development").expect("ok"));
    }

    #[test]
    fn given_cors_list_when_parsing_then_entries_are_trimmed_and_exact() {
        let origins = parse_cors_origins(
            "http://127.0.0.1:3910 ; https://a.example.com,app://dsh,,https://servicewechat.com",
        );
        assert_eq!(
            origins,
            vec![
                "http://127.0.0.1:3910",
                "https://a.example.com",
                "app://dsh",
                "https://servicewechat.com"
            ]
        );
    }
}
