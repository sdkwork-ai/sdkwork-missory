//! Standalone gateway for the sdkwork-missory application HTTP plane.
//!
//! Process concerns only: fail-closed environment validation, static console
//! hosting, binding, and graceful shutdown. Authentication and route/IAM
//! composition live in the owner assembly
//! (`sdkwork_api_missory_assembly::compose_authenticated_router_from_env`):
//! IAM dual-token requests resolve through the shared web-framework pipeline
//! and anonymous development requests receive the bypass identity
//! (`TECH_ARCHITECTURE.md` §8 item 1).

use std::time::Duration;

use axum::extract::Request;
use axum::http::{header, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::Router;

/// Environment variables governing startup.
const ENVIRONMENT_KEY: &str = "SDKWORK_MISSORY_ENVIRONMENT";
const RUNTIME_TARGET_KEY: &str = "SDKWORK_MISSORY_RUNTIME_TARGET";
const BIND_KEY: &str = "SDKWORK_MISSORY_APPLICATION_PUBLIC_INGRESS_BIND";
/// Dev-only identity bypass (`CORS_SPEC.md`/ENVIRONMENT_SPEC: development topologies only).
const DEV_AUTH_BYPASS_KEY: &str = "SDKWORK_MISSORY_DEV_AUTH_BYPASS";
/// Optional static web console directory; when set, the gateway hosts the
/// built browser app same-origin (standalone single-process deployment).
const STATIC_DIR_KEY: &str = "SDKWORK_MISSORY_STATIC_DIR";

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
/// without a provisioned IAM database. It MUST be false in every
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

/// SPA history fallback: GET requests to extension-less, non-API paths that
/// routed nowhere (404 from the static file service) return the console
/// `index.html` with 200 so client-side routing owns deep links.
async fn spa_history_fallback(
    axum::extract::State(index_path): axum::extract::State<std::path::PathBuf>,
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let method = request.method().clone();
    let response = next.run(request).await;
    if response.status() != StatusCode::NOT_FOUND {
        return response;
    }
    let path = path.as_str();
    let is_reserved = path.starts_with("/app/")
        || path.starts_with("/healthz")
        || path.starts_with("/readyz")
        || path.starts_with("/livez")
        || path.starts_with("/metrics")
        || path == "/runtime-env.json";
    let last_segment = path.rsplit('/').next().unwrap_or("");
    if method != Method::GET || is_reserved || last_segment.contains('.') {
        return response;
    }
    match tokio::fs::read(&index_path).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], bytes).into_response(),
        Err(_) => response,
    }
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

/// Mounts the built web console when `SDKWORK_MISSORY_STATIC_DIR` is set:
/// static files + SPA history fallback (reserved health/API paths excluded).
fn mount_static_console_if_configured(mut router: Router) -> Option<Router> {
    let Ok(static_dir) = std::env::var(STATIC_DIR_KEY) else {
        return Some(router);
    };
    if static_dir.trim().is_empty() {
        return Some(router);
    }
    let index_path = std::path::Path::new(&static_dir).join("index.html");
    if !index_path.exists() {
        tracing::error!(
            static_dir = %static_dir,
            "SDKWORK_MISSORY_STATIC_DIR has no index.html; refusing static hosting"
        );
        return None;
    }
    tracing::info!(static_dir = %static_dir, "hosting web console same-origin");
    router = router
        .fallback_service(
            tower_http::services::ServeDir::new(&static_dir)
                .append_index_html_on_directories(false),
        )
        .layer(middleware::from_fn_with_state(
            index_path,
            spa_history_fallback,
        ));
    Some(router)
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
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
            return std::process::ExitCode::from(2);
        }
    };
    let dev_bypass = match resolve_dev_bypass(&environment) {
        Ok(value) => value,
        Err(reason) => {
            tracing::error!("{reason}");
            return std::process::ExitCode::from(2);
        }
    };
    tracing::info!(environment = %environment, dev_bypass,
        "starting sdkwork-missory standalone gateway");

    if matches!(std::env::args().nth(1).as_deref(), Some("db-migrate")) {
        let code = sdkwork_api_missory_assembly::run_database_migrate_only().await;
        return std::process::ExitCode::from(code);
    }
    if matches!(std::env::args().nth(1).as_deref(), Some("issue-bootstrap-token")) {
        let args = std::env::args().collect::<Vec<_>>();
        let tenant = args
            .iter()
            .position(|a| a == "--tenant")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "100001".to_owned());
        let app = args
            .iter()
            .position(|a| a == "--app")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "sdkwork-missory".to_owned());
        let issued = sdkwork_api_missory_assembly::issue_standalone_bootstrap_credential(&tenant, &app)
            .await;
        return match issued {
            Ok(value) => {
                println!("{}", serde_json::to_string_pretty(&value).unwrap_or_default());
                std::process::ExitCode::SUCCESS
            }
            Err(reason) => {
                tracing::error!("issue bootstrap credential: {reason}");
                std::process::ExitCode::from(2)
            }
        };
    }

    let authenticated_router =
        match sdkwork_api_missory_assembly::compose_authenticated_router_from_env(dev_bypass).await
        {
            Ok(router) => router,
            Err(reason) => {
                tracing::error!("compose authenticated router: {reason}");
                return std::process::ExitCode::from(2);
            }
        };
    let router = sdkwork_web_bootstrap::service_router(
        authenticated_router,
        sdkwork_web_bootstrap::ServiceRouterConfig::default().with_always_ready(),
    );

    let Some(router) = mount_static_console_if_configured(router) else {
        return std::process::ExitCode::from(2);
    };

    let bind = std::env::var(BIND_KEY).unwrap_or_else(|_| default_bind().to_string());
    let listener = match tokio::net::TcpListener::bind(&bind).await {
        Ok(listener) => listener,
        Err(error) => {
            tracing::error!("failed to bind {bind}: {error}");
            return std::process::ExitCode::from(2);
        }
    };
    tracing::info!(bind = %bind, "missory app-api listening");
    if let Err(error) = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        tracing::error!("server error: {error}");
        return std::process::ExitCode::from(1);
    }
    std::process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    // Env-var mutations race across parallel test threads; keep the bypass
    // matrix in one serialized test.
    #[test]
    fn given_bypass_matrix_when_resolved_then_fail_closed_outside_development() {
        std::env::set_var(DEV_AUTH_BYPASS_KEY, "true");
        assert!(
            resolve_dev_bypass("production").is_err(),
            "bypass must fail closed outside development"
        );
        assert!(resolve_dev_bypass("development").expect("allowed"));
        std::env::remove_var(DEV_AUTH_BYPASS_KEY);
        assert!(!resolve_dev_bypass("development").expect("ok"));
    }
}
