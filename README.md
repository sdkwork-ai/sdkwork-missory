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
| `apps/` | Application roots (client surfaces; none shipped in the current phase) |
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

## Quick Start

```bash
pnpm install
pnpm build          # cargo build --workspace
pnpm test           # cargo test --workspace
pnpm check          # composition + standards gates + cargo check
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
