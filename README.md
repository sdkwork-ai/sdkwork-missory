# SDKWork Missory (念忆)

repository-kind: application

> **记住每一个重要的人。**

Missory is an AI personal social memory application: it helps a user remember people,
understand relationships, keep structured memories, organize shared stories, and act with
AI assistance — always draft-only, never auto-contacting anyone.

## Documentation Canon

- [docs/README.md](docs/README.md) — documentation index and audience routing
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md) — product requirements (念忆 · Missory PRD V1.0)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md) — technical architecture

## Repository Layout

Standard SDKWork application repository dictionary:

| Directory | Content |
| --- | --- |
| `apis/` | Author-owned OpenAPI authority inputs |
| `apps/` | Client surfaces: PC console (React + Electron desktop host), H5 mobile web, WeChat mini program, Flutter mobile |
| `crates/` | Rust workspace crates (contract, SPI, service, routes, assembly, gateway, test support) |
| `plugins/` | Rust plugin crates implementing SPI ports (in-memory store) |
| `sdks/` | SDK families and materialized route manifests |
| `jobs/` | Scheduled background jobs |
| `tools/` | Repository-owned Node.js helper scripts |
| `etc/` | Source-controlled deployment/runtime config templates |
| `deployments/` | Deployment environment descriptors |
| `scripts/` | Thin local command entrypoints |
| `docs/` | Canon documentation |
| `tests/` | Repository-level contract assets |
| `bin/` | Standardized module operator entrypoints |
| `specs/` | Repository component and topology contracts |
| `.sdkwork/` | Source-controlled workspace metadata |

## Crates

| Crate | Role |
| --- | --- |
| `sdkwork-missory-contract` | HTTP DTOs, service ports, request context, typed service errors |
| `sdkwork-missory-spi` | Store ports (person/relationship/memory/story/reminder persistence boundary) |
| `sdkwork-communication-missory-service` | Business logic: people, relationships, memories, stories, reminder derivation, rule-based AI assistant (pluggable `LanguageModelPort`) |
| `sdkwork-missory-plugin-store-memory` | In-memory SPI store implementation (phase-1 persistence adapter) |
| `sdkwork-routes-missory-app-api` | App API HTTP surface (`/app/v3/api/missory/*`) |
| `sdkwork-api-missory-assembly` | Materialized API assembly for the application HTTP plane |
| `sdkwork-api-missory-standalone-gateway` | Standalone gateway binary (axum host) |
| `sdkwork-missory-test-support` | Shared test fixtures |

## Client Surfaces

| Surface | Root | Stack | Build |
| --- | --- | --- | --- |
| PC console | `apps/sdkwork-missory-pc` | React 19 + Vite 8 + Tailwind 4 | `pnpm build:pc:<dev\|test\|staging\|demo\|prod>[:cloud]` |
| Desktop host | `apps/sdkwork-missory-pc/packages/sdkwork-missory-pc-electron` | Electron wrapping the PC dist | `pnpm --dir apps/sdkwork-missory-pc/packages/sdkwork-missory-pc-electron package` |
| H5 mobile web | `apps/sdkwork-missory-h5` | React 19 + HashRouter | `pnpm build:h5:<env>[:cloud]` |
| WeChat mini program | `apps/sdkwork-missory-mini-program` | Native WXML + esbuild runtime bundle | `pnpm --dir apps/sdkwork-missory-mini-program build:mini-program[:staging\|:prod]` |
| Flutter mobile | `apps/sdkwork-missory-flutter-mobile` | Flutter (Material 3) + generated Dart SDK | `flutter build apk --dart-define-from-file=env/sdkwork.<profile>.<env>.json` |

All surfaces consume the generated `sdkwork-missory-app-sdk` family
(Typescript `@sdkwork/missory-app-sdk`, Dart `sdkwork_missory_app_sdk`) against
the locked `/app/v3/api` prefix. The standalone gateway hosts the built
console same-origin (`SDKWORK_MISSORY_STATIC_DIR`) with health probes at
`/healthz` and `/readyz`.

## Quick Start

```bash
pnpm install
pnpm build          # cargo build --workspace
pnpm test           # cargo test --workspace
pnpm check          # composition + standards gates + cargo check

# PC console against the local gateway:
pnpm dev            # standalone topology (gateway + PC renderer)

# Flutter app against the local gateway (Android emulator loopback):
cd apps/sdkwork-missory-flutter-mobile
flutter run --dart-define-from-file=env/sdkwork.standalone.development.json
```

Run the standalone gateway directly:

```bash
pnpm gateway:run:standalone
# or, with explicit development environment:
SDKWORK_MISSORY_ENVIRONMENT=development cargo run -p sdkwork-api-missory-standalone-gateway
```

The gateway binds `127.0.0.1:8080` by default (`SDKWORK_MISSORY_APPLICATION_PUBLIC_INGRESS_BIND`).

## Standards

This repository follows the global standards in `../sdkwork-specs/` (single source of
truth; never copied locally). The agent entrypoint is [AGENTS.md](AGENTS.md).
