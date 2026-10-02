# Repository Guidelines (apps/sdkwork-missory-pc)

Read `../../../AGENTS.md` first, then `../../../sdkwork-specs/SOUL.md` and the
task-specific specs under `../../../sdkwork-specs/` (APP_PC_ARCHITECTURE_SPEC.md,
APP_PC_REACT_UI_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, THEME_DARKMODE_SPEC.md).

## Local Dictionary

- `src/`: thin app root (bootstrap, router assembly, Tailwind entry).
- `packages/`: `sdkwork-missory-pc-core` (runtime config + SDK client factory +
  services), `-commons` (presentation primitives), `-shell` (layout + home),
  `-people` / `-memories` / `-assistant` (capability screens).
- `etc/`: component deployment index and browser runtime-env source documents.
- `sdks/` (repo root): generated `@sdkwork/missory-app-sdk` consumer facade.

## Rules

- SDK clients are constructed only in `-core`; screens receive services through
  the bootstrapped runtime. Never import `generated/server-openapi` directly.
- Business calls go through the generated SDK client; no raw fetch for
  Missory resources.
- No `resolve.alias` for package specifiers in Vite config.
- Build outputs go to `dist/<deploymentProfile>/<environment>` via the canonical
  `resolveBrowserDistOutDir` helper only.
