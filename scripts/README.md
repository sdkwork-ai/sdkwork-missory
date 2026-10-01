# scripts/

## Purpose

Thin local command entrypoints that delegate to Cargo and the standards tools; keep them minimal.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

Small `*.mjs`/`*.sh` wrappers invoked by root `package.json` scripts.

## Forbidden content

Long business logic, inline secrets, deletion by wildcard (forbidden by `../sdkwork-specs/DESTRUCTIVE_OPERATION_SPEC.md`).

## Related specs

- `../sdkwork-specs/CODE_STYLE_SPEC.md`\n- `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-script-placement.mjs --workspace ..`
