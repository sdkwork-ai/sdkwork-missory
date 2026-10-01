# crates/

## Purpose

All Rust crates for the Missory backend: missory contract/SPI crates, the domain service crate, route crates, the API assembly, the standalone gateway, and test support.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

One directory per Cargo package named exactly like its `[package].name`, each with `Cargo.toml`, `README.md`, `specs/component.spec.json`, `src/`, and `tests/`.

## Forbidden content

Node packages, generated SDK output, unchecked-in generated manifests (route manifests materialize under `sdks/_route-manifests/`).

## Related specs

- `../sdkwork-specs/RUST_CODE_SPEC.md`\n- `../sdkwork-specs/NAMING_SPEC.md` section 3.1/3.2\n- `../sdkwork-specs/APPLICATION_LAYERED_ARCHITECTURE_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-rust-crate-naming-standard.mjs --root .` && `cargo test --workspace`
