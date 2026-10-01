# sdkwork-api-missory-standalone-gateway

Standalone gateway binary hosting the Missory app-api plane. Startup is
fail-closed: `SDKWORK_MISSORY_ENVIRONMENT` must name an explicit environment
(see `.env.example`). The binary only owns process concerns — binding, tracing,
context injection, graceful shutdown; routing and business wiring live in the
assembly crate.

Canonical specs: `../../../sdkwork-specs/RUST_CODE_SPEC.md`,
`../../../sdkwork-specs/API_ASSEMBLY_SPEC.md`.
