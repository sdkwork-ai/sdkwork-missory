# plugins/

Rust plugin crates implementing missory SPI ports.

Allowed content: one directory per Cargo package named exactly like its
`[package].name` (e.g. `sdkwork-missory-plugin-store-memory`), each with
`Cargo.toml`, `README.md`, `specs/component.spec.json`. Forbidden: non-plugin
crates, generated output. Related specs: `../sdkwork-specs/RUST_CODE_SPEC.md`.
Verification: `node ../sdkwork-specs/tools/check-rust-crate-naming-standard.mjs --root .`
