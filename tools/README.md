# tools/

## Purpose

Repository-owned Node.js helper scripts (for example standalone gateway launcher and repository checks).

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

`*.mjs` scripts with a clear CLI contract, plus their tests.

## Forbidden content

Business logic, secrets, environment-specific values.

## Related specs

- `../sdkwork-specs/CODE_STYLE_SPEC.md` section 7

## Verification

`node --test tools/*.test.mjs` (when test files exist)
