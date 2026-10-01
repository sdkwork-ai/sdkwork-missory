# jobs/

## Purpose

Scheduled background jobs. Reserved in the current phase; future reminder dispatch workers live here.

## Owner

sdkwork-missory platform team (application code `missory`).

## Allowed content

Job definitions under `schedules/` with their own README and manifests.

## Forbidden content

Inline business logic that belongs in service crates.

## Related specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`

## Verification

Manual review against `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md` section 1.1.
