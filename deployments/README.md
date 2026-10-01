# deployments/

## Purpose

Deployment environment descriptors and the artifacts output area for packaged bundles.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

Environment descriptors, `.gitkeep` placeholder for `artifacts/`.

## Forbidden content

Tracked build artifacts, secrets, environment-specific credentials.

## Related specs

- `../sdkwork-specs/DEPLOYMENT_SPEC.md`\n- `../sdkwork-specs/APPLICATION_DEPLOY_LAYOUT_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-deploy-standard.mjs`
