# Missory Technical Architecture

- Status: active
- Owner: sdkwork-missory platform team
- Updated: 2026-10-01
- Specs: `../../../sdkwork-specs/ARCHITECTURE_DECISION_SPEC.md`, `../../../sdkwork-specs/RUST_CODE_SPEC.md`, `../../../sdkwork-specs/APPLICATION_LAYERED_ARCHITECTURE_SPEC.md`, `../../../sdkwork-specs/API_SPEC.md`

## 1. Overview

Missory is a Rust-first backend application (application code `missory`, domain
`communication`, capability `missory`) exposing an app-api HTTP surface under
`/app/v3/api/missory/*`. The product object model is people → relationships → memories →
stories, with derived reminders and a draft-only social AI assistant.

## 2. Crate Composition

```text
crates/
  sdkwork-missory-contract            # L2/L3 contracts: DTOs, service ports, context, errors
  sdkwork-missory-spi                 # store ports (persistence boundary)
  sdkwork-communication-missory-service  # L2/L3 service: use cases + reminder engine + assistant
  sdkwork-routes-missory-app-api      # L1 HTTP adapter (axum), app-api surface
  sdkwork-api-missory-assembly        # materialized API assembly (runtime composition)
  sdkwork-api-missory-standalone-gateway  # standalone gateway binary
  sdkwork-missory-test-support        # shared fixtures
plugins/
  sdkwork-missory-plugin-store-memory # L4 store adapter (in-memory)
```

Dependency direction follows `APPLICATION_LAYERED_ARCHITECTURE_SPEC.md`:
routes → (service ports from contract) ← service → (store ports from SPI) ← store adapter.
Route crates never touch store types; service crates never touch HTTP types; store adapters
never touch HTTP framework crates.

## 3. HTTP Surface

- Authority: `sdkwork-missory-app-api`, prefix `/app/v3/api` (locked app-api prefix), resource
  segment `/missory/...`.
- Envelope: `SdkWorkApiResponse { code, data, traceId }` from `sdkwork-utils-rust`;
  single resources as `data.item`, lists as `data.items` + `data.pageInfo` (offset mode),
  commands as `data.accepted`.
- Errors: RFC 9457 `application/problem+json` with numeric `code` and server-minted `traceId`.
- `int64` wire fields serialize as JSON strings (`serde_uint64` helpers).
- Operation matrix (API_SPEC §15.4): create `201`, retrieve/list/search `200`, update `200`,
  delete `204` (no body), commands `POST /{id}/{action}` returning `SdkWorkCommandData`.

## 4. Domain Model (Phase 1)

- `Person`: display name, aliases, gender, birthday, city, title, company, avatar, tags,
  interests, preferences, bio, contact channels, notes.
- `Relationship`: person ↔ owner types (multiple), started-at, last-contacted-at,
  description, importance, contact cycle days, notes.
- `Memory`: typed (semantic/episodic/temporal/relationship/preference/commitment), content,
  `origin = fact | inference` (inference carries confidence/source/reason), lifecycle status
  (`candidate/confirmed/rejected/archived/expired`), importance, occurrence date, person link,
  story link, source type.
- `Story`: title, summary, participants, event/memory ids, time range; AI summary via
  assistant port.
- `ReminderState`: dismiss/snooze bookkeeping for derived reminders (long-uncontacted,
  birthday, commitment, important event).

## 5. AI Assistant

`LanguageModelPort` (SPI trait) abstracts text generation. Phase 1 ships a deterministic
rule-based implementation (`RuleBasedSocialAgent`) that answers person/relationship/memory
questions from the store, builds meeting briefings, drafts messages in preset tones, and
summarizes pasted chat text into memory **candidates**. A real provider can be plugged in
later without touching routes or domain logic. All AI output remains draft/candidate-only.

## 6. Persistence

The authoritative store is `sdkwork-communication-missory-repository-sqlx`: a PostgreSQL
(sqlx) implementation of the SPI ports over the `missory_` tables declared in
`database/database.manifest.json` (authoritative-server, baseline-plus-migrations). The
database host performs manifest engine admission, a PostgreSQL 15+ gate, lifecycle
init/migrate (`SDKWORK_DATABASE_AUTO_MIGRATE`, `db-migrate` argv mode), and snowflake node
allocation. The in-memory store plugin (`sdkwork-missory-plugin-store-memory`) remains the
development/test adapter; the gateway selects fail-closed
(`SDKWORK_MISSORY_STORE=memory|postgres`, or via `SDKWORK_DATABASE_*` presence —
production-like environments refuse the in-memory adapter).

## 7. Configuration And Runtime

- Environment variables use the `SDKWORK_MISSORY_*` namespace (see `.env.example`).
- `SDKWORK_MISSORY_ENVIRONMENT` gates server startup: the standalone gateway refuses to boot
  without an explicit `development|test|staging|demo|production` value (fail-closed), with
  `SDKWORK_MISSORY_RUNTIME_TARGET=test-runner` as the test escape hatch.
- Bind address: `SDKWORK_MISSORY_APPLICATION_PUBLIC_INGRESS_BIND` (default `127.0.0.1:8080`).
- Deployment profiles: `standalone` and `cloud` (see `sdkwork.app.config.json` and
  `etc/sdkwork.deployment.config.json`).

## 8. Adoption Debt (Explicit)

The following fleet integrations are intentionally deferred and tracked as debt; none of
them changes the crate boundaries above:

1. ~~IAM dual-token adapter~~ **Adopted (2026-10-02,
   `docs/architecture/decisions/ADR-20261002-iam-dual-token-adoption.md`)**: the assembly
   wires the `sdkwork-iam-web-adapter` resolver + the shared web-framework pipeline
   (`WebRequestContext` resolver + domain injector) behind the same
   `MissoryRequestContext` contract; raw identity-header trust is removed in every
   environment and the dev bypass survives only as a development-only inner fallback.
2. ~~Generated TypeScript/Dart SDK consumption of IAM login flows~~ **Adopted
   (2026-10-02, same ADR)**: PC/H5/mini-program/Flutter all ship the credential-entry
   login flow, persisted dual-token sessions, session gates, and 401 expiry boundaries
   through the generated SDK token managers.
3. Background reminder dispatch worker under `jobs/` (reminders are computed on read).
4. ~~Production-grade E2E login run against a provisioned IAM database~~ **Performed
   live (2026-10-02)** against the workspace development PostgreSQL: gateway boot
   (`SDKWORK_DATABASE_URL` + dev environment bridge), `issue-bootstrap-token`,
   credential-entry registration + password login, dual-token business calls resolving
   the real principal (`my_profile.userId` = the registered snowflake id), person
   create/list scoped per user, and the negative matrix (no credentials 401, spoofed
   legacy identity headers rejected 400 by the forbidden-header guard, garbage tokens
   401, second user sees zero of user-one's rows). **Repeatable since 2026-10-03**:
   `pnpm run test:e2e-login` (`scripts/e2e/login-flow.e2e.mjs`) automates the chain and
   additionally asserts memory extraction/confirm domain rules and the whole-account
   privacy export (`POST /app/v3/api/missory/data_exports`, PRD §9) with cross-user
   isolation.

Each adoption lands with its own ADR under `docs/architecture/decisions/`.
