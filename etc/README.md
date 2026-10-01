# etc/

## Purpose

Source-controlled deployment/runtime config templates: dependency refs, deployment config, topology env files, and config examples.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

`README.md`, `sdkwork.deployment.config.json`, `dependency-refs.json`, `topology/*.env`, `examples/`.

## Forbidden content

Real secrets, machine-local absolute paths, generated artifacts.

## Related specs

- `../sdkwork-specs/SOURCE_CONFIG_SPEC.md`\n- `../sdkwork-specs/CONFIG_SPEC.md`\n- `../sdkwork-specs/ENVIRONMENT_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-workspace-path-portability.mjs --root .`
