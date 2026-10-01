# sdks/

## Purpose

SDK family workspaces and materialized route manifests (`_route-manifests/`) derived from the owned route crates.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

Generated SDK families (`sdkwork-missory-app-sdk`), materialized `*.route-manifest.json` files, SDK manifests.

## Forbidden content

Hand-edited generated output, business logic.

## Related specs

- `../sdkwork-specs/SDK_SPEC.md`\n- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`

## Verification

`pnpm api:assembly:validate`
