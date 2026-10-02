//! Development console runtime-env injection (credential-entry bootstrap).
//!
//! Same-origin browser consoles fetch `/runtime-env.json` from the gateway and
//! pass `authBootstrapAccessToken` to the credential-entry login/registration
//! calls (`BROWSER_RUNTIME_ENV_SPEC` leaves that field to deployment-time
//! injection). Development builds carry no injected token, so the gateway
//! injects the deployment-resolved credential here: the same signed,
//! tenant-bound JWT the operator CLI issues via `issue-bootstrap-token`,
//! resolved once per process and served only by the development route.

use tokio::sync::OnceCell;

static BOOTSTRAP_ACCESS_TOKEN: OnceCell<Option<String>> = OnceCell::const_new();

/// Console runtime application key; the bootstrap credential's `appId` must
/// match the missory app-api runtime audience or the credential-entry guard
/// rejects it.
const CONSOLE_RUNTIME_APP_ID: &str = "sdkwork-missory";

async fn deployment_bootstrap_access_token() -> Option<String> {
    BOOTSTRAP_ACCESS_TOKEN
        .get_or_init(|| async {
            match sdkwork_iam_web_adapter::resolve_deployment_bootstrap_access_token(
                None,
                Some(CONSOLE_RUNTIME_APP_ID),
            )
            .await
            {
                Ok(token) => token,
                Err(reason) => {
                    tracing::warn!("resolve console bootstrap access token failed: {reason}");
                    None
                }
            }
        })
        .await
        .clone()
}

/// Reads the console `runtime-env.json` document and injects the
/// deployment-provisioned bootstrap `Access-Token`.
///
/// `None` when the static document is absent/invalid or no bootstrap
/// credential could be resolved; the caller then falls back to serving the
/// static file unchanged.
pub async fn development_console_runtime_env_json(static_dir: &str) -> Option<String> {
    let document = std::fs::read_to_string(format!("{static_dir}/runtime-env.json")).ok()?;
    let mut environment: serde_json::Value = serde_json::from_str(document.trim()).ok()?;
    let Some(object) = environment.as_object_mut() else {
        return None;
    };
    let token = deployment_bootstrap_access_token().await?;
    object.insert(
        "authBootstrapAccessToken".to_owned(),
        serde_json::Value::String(token),
    );
    Some(environment.to_string())
}
