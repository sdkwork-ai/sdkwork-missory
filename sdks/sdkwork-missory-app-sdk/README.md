# sdkwork-missory-app-sdk

SDK family for the Missory (念忆) app-api authority `sdkwork-missory-app-api`
(prefix `/app/v3/api`). Generated output is owned by `@sdkwork/sdk-generator`
(`sdkgen`); never hand-edit `*/generated/server-openapi`.

- Authority: `openapi/sdkwork-missory-app-api.openapi.yaml` (synced from
  `apis/app-api/communication/missory-app-api.openapi.yaml`)
- Generator input: `openapi/sdkwork-missory-app-api.sdkgen.yaml`
- Languages: TypeScript (composed consumer `@sdkwork/missory-app-sdk`),
  Dart (`sdkwork_missory_app_sdk`, consumed by the Flutter mobile app)
- Standard profile: `sdkwork-v3`

Regenerate: `pnpm sdk:generate` — Verify: `pnpm sdk:check`.

Related specs: `../../sdkwork-specs/SDK_SPEC.md`,
`../../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`.
