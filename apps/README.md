# apps/

## Purpose

Application roots for client surfaces. No client app is shipped in the current phase; the directory is reserved per the repository directory dictionary.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

Application roots named `sdkwork-missory-<surface>` with their own manifests and README.

## Forbidden content

Backend Rust code (belongs in `crates/`), generated SDK output (belongs in `sdks/`).

## Related specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`\n- `../sdkwork-specs/NAMING_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-workspace-layout.mjs --root .`
