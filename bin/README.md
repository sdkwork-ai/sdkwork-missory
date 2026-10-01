# bin/

## Purpose

Standardized module operator entrypoints (nine scripts per MODULE_BIN_SPEC): docker-image, docker-deploy, apps-build, apps-package, apps-deploy, apps-pkg-installer, config, doctor, backup.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

The nine standard `*.sh` entrypoints plus `lib/` helpers delegating to `../sdkwork-specs/bin/lib/sdkwork-common.sh`.

## Forbidden content

Long inline implementation (delegate to the shared library), wildcard deletion, secrets.

## Related specs

- `../sdkwork-specs/MODULE_BIN_SPEC.md`\n- `../sdkwork-specs/PORTABILITY_SPEC.md`

## Verification

`node ../sdkwork-specs/tools/check-module-bin.mjs --root .`
