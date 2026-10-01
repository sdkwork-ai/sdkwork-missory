# tests/

## Purpose

Repository-level contract assets and fixtures shared across crates.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

Contract fixtures, cross-crate integration assets.

## Forbidden content

Unit tests (those live inside each crate's `tests/` or `#[cfg(test)]` modules), secrets.

## Related specs

- `../sdkwork-specs/TEST_SPEC.md`\n- `../sdkwork-specs/RUST_CODE_SPEC.md` section 12

## Verification

`cargo test --workspace`
