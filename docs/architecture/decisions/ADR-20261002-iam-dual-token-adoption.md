# ADR 0001: IAM Dual-Token Authentication Adoption

- Status: Accepted (2026-10-02)
- Deciders: sdkwork-missory maintainers
- Authority: `docs/architecture/tech/TECH_ARCHITECTURE.md` §8 (debt items 1–2),
  `../sdkwork-specs/API_SPEC.md`, `../sdkwork-specs/WEB_BACKEND_SPEC.md`

## Context

Before this ADR the standalone gateway resolved request identity by trusting the
raw `x-sdkwork-user-id` / `x-sdkwork-tenant-id` headers in every environment,
plus a development bypass identity. The header path was a pre-IAM local-topology
shortcut (TECH_ARCHITECTURE §8 item 1) and is a spoofing hole in any deployed
environment: those headers sit on the web framework's forbidden client-identity
projection list. Production also had no login surface at all.

## Decision

1. **Server — resolver + domain injector behind the same domain contract.**
   The assembly (`sdkwork-api-missory-assembly::auth`) builds an audience-bound
   `sdkwork-iam-web-adapter` resolver over the application PostgreSQL database
   (one database hosts both `iam_*` and missory tables, matching the fleet
   community gateway shape), wraps the missory app-api business router with the
   shared web-framework pipeline
   (`wrap_router_with_iam_app_web_framework_resolver` + the authored
   `gateway_route_manifest()`), and a `MissoryContextInjector` projects the
   resolved `WebRequestPrincipal` into the same `MissoryRequestContext`
   extension handlers have always consumed. The self-contained IAM app-api
   router (login / registration / session surface, framework-wrapped by the IAM
   crate) merges at the top level, so the standalone gateway serves credential
   entry same-origin.

2. **Security posture.** Raw identity-header trust is removed in every
   environment. The development bypass survives only as an inner fallback
   middleware that runs after real dual-token resolution and is refused at
   startup outside `development`. Non-development environments require
   PostgreSQL (IAM plane boots from it) and answer 401 without valid IAM
   credentials; production additionally enforces the adapter's
   `assert_production_hardening` (`SDKWORK_IAM_SIGNING_MASTER_SECRET` required;
   dev-fallback, super-admin password, and OAuth secret env overrides
   forbidden) plus the audience claim policy for `sdkwork-missory` / `missory`.

3. **Login contract.** `POST /app/v3/api/auth/sessions` with
   `{grantType: "password", username, password}` and the deployment-provisioned
   bootstrap `Access-Token` header (credential-entry routes reject
   `Authorization`). The response `{code: 0, data: {accessToken, authToken,
   refreshToken, user, context}}` feeds the shared token manager; every
   subsequent request carries `Authorization: Bearer <authToken>` +
   `Access-Token: <accessToken>`.

4. **Clients — all four surfaces.** PC (`sdkwork-missory-pc-core`) and H5
   (`sdkwork-missory-h5-core`) gain a persisted session store (localStorage),
   a session facade (login/logout/isAuthenticated) bound to the shared
   `AuthTokenManager`, a credential-entry `LoginScreen`, a logout control in
   the app shell, and a session gate that renders login until a dual-token
   session exists (development with the gateway bypass stays signed-in by
   default) plus a fetch-level 401/403 boundary that clears the session. The
   WeChat mini program (`sdkwork-missory-mp-core`) mirrors the facade with
   WeChat storage persistence, a native login page, an app-launch gate, and a
   runtime-bundle bootstrap-token placeholder. Flutter
   (`sdkwork_missory_flutter_mobile_core`) adds a `MissorySession`
   (shared_preferences persistence) projected into the generated client via
   `setAuthToken`/`setAccessToken`, a Material login screen, an app-level gate,
   and a home-screen logout action. Generated SDK output is untouched.

5. **Bootstrap token provisioning.** The credential-entry bootstrap
   `Access-Token` is deployment configuration, never source: browsers read the
   optional `authBootstrapAccessToken` runtime-env field (injected at
   deployment), Flutter reads the `SDKWORK_MISSORY_AUTH_BOOTSTRAP_ACCESS_TOKEN`
   dart-define, and the mini program reads the same-named runtime-env key at
   bundle time. Development resolves any value through the IAM dev
   authentication fallback.

## Consequences

- Every environment authenticates against real IAM sessions; identity spoofing
  via client headers is impossible (the framework also rejects those headers
  outright).
- `pnpm dev` keeps working without a database (memory store + bypass); with
  `SDKWORK_DATABASE_URL` set, the gateway boots the IAM plane and real login
  works end to end, including in development.
- Operators must provision `SDKWORK_DATABASE_URL` (production required), the
  signing master secret, and a credential-entry bootstrap `Access-Token` for
  each deployed client surface before first traffic.
- Remaining debt: the background reminder dispatch worker (TECH_ARCHITECTURE
  §8 item 3) and a production-grade E2E login run against a provisioned
  database are tracked in the tech-architecture debt list / runbook.

## Verification

- `cargo test --workspace` (route manifest, injector, bypass matrix, assembly
  composition tests)
- `pnpm verify` + repository spec gates (`node ../sdkwork-specs/tools/*.mjs`)
- Client checks: `pnpm --dir apps/sdkwork-missory-pc check`,
  `pnpm --dir apps/sdkwork-missory-h5 check`,
  `pnpm --dir apps/sdkwork-missory-mini-program check`,
  `flutter analyze && flutter test` (Flutter mobile), desktop host check.
