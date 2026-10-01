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

Phase 1 uses the in-memory store plugin (`sdkwork-missory-plugin-store-memory`) implementing
the SPI ports with tenant-scoped maps — this keeps the service contract stable while the SQL
adapter is built. Phase 2 introduces `sdkwork-communication-missory-repository-sqlx` with
PostgreSQL as the authoritative engine per `DATABASE_SPEC.md` (missory_ table prefix,
standard audit/subject columns, `database/` lifecycle assets, `db:*` CLI wiring).

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

1. `sdkwork-web-framework` router wrapping and IAM dual-token context adapter (routes
   currently resolve the request context from a dedicated extension; the contract shape
   matches `WebRequestContext` fields so the adapter is additive).
2. SQLx/PostgreSQL repository crate and `database/` lifecycle assets.
3. Generated TypeScript SDK family under `sdks/`.
4. Background reminder dispatch worker under `jobs/`.

Each adoption lands with its own ADR under `docs/architecture/decisions/`.
