# Repository Guidelines (apps/sdkwork-missory-mini-program)

Read `../../../AGENTS.md` first, then `../../../sdkwork-specs/SOUL.md` and the
task-specific specs under `../../../sdkwork-specs/` (MINI_PROGRAM_APP_ARCHITECTURE_SPEC.md, APP_MINI_PROGRAM_UI_SPEC.md, APP_SDK_INTEGRATION_SPEC.md).

## Local Dictionary

- `src/`: thin app root (`app.js`/`app.json`/pages WXML; bootstrap under `src/bootstrap/`).
- `packages/`: `sdkwork-missory-mp-core` (generated SDK client + services), `-commons` (i18n fragments), `-host` (wx.request→fetch adapter).
- `config/mini-program/`: per-profile runtime-env sources; `scripts/build-runtime.mjs` bundles `src/runtime/runtime.js`.
- `sdks/` (repo root): generated `@sdkwork/missory-app-sdk` facade.

## Surface Rules

- Pages consume services through the bundled runtime; never call `wx.request` directly.
- The runtime bundle is a build artifact: run `pnpm build:mini-program` before opening WeChat devtools.
