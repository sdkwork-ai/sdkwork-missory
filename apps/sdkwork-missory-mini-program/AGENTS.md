# Repository Guidelines (apps/sdkwork-missory-mini-program)

Read `../../../AGENTS.md` first, then `../../../sdkwork-specs/SOUL.md` and the
task-specific specs under `../../../sdkwork-specs/` (MINI_PROGRAM_APP_ARCHITECTURE_SPEC.md, APP_MINI_PROGRAM_UI_SPEC.md, APP_SDK_INTEGRATION_SPEC.md).

## Local Dictionary

- `src/`: thin app root (`app.ts`/`app.json`/pages `index.ts` + WXML/WXSS;
  bootstrap under `src/bootstrap/`, page-facing runtime typing under `src/typings/`).
- `packages/`: `sdkwork-missory-mp-core` (generated SDK client + services), `-commons` (i18n fragments + error primitives), `-host` (wx.request→fetch adapter).
- `config/mini-program/`: per-profile runtime-env sources; `scripts/build-runtime.mjs` bundles `src/runtime/runtime.js`.
- `sdks/` (repo root): generated `@sdkwork/missory-app-sdk` facade.

## Surface Rules

- All authored sources are TypeScript (`MINI_PROGRAM_APP_ARCHITECTURE_SPEC.md`
  language boundary): `src/app.ts`, pages `index.ts`, `packages/**` `.ts`.
  The only JavaScript under `src/` is the generated runtime bundle.
- WeChat DevTools compiles `src/` TypeScript through the `typescript`
  compiler plugin (`useCompilerPlugins` in `project.config.json`); page
  sources may only `import type` from `src/typings/` (erased at compile) and
  reach runtime services through the bundled runtime module.
- Pages consume services through the bundled runtime; never call `wx.request` directly.
- The runtime bundle is a build artifact: run `pnpm build:mini-program` before opening WeChat devtools.
- `pnpm typecheck` runs `tsc --noEmit` strict over the whole authored graph
  (`src/**/*.ts` + `packages/*/src/**/*.ts`).
