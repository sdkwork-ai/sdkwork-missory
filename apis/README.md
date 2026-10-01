# apis/

## Purpose

Author-owned OpenAPI authority inputs for the Missory HTTP surfaces. The app-api authority describes the /app/v3/api/missory/* contract.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

OpenAPI YAML/JSON authority files organized by surface/domain (apis/app-api/communication/), shared component schemas.

## Forbidden content

Generated SDK output, route crates, source code, environment files.

## Related specs

- `../sdkwork-specs/API_SPEC.md`\n- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --root .`
