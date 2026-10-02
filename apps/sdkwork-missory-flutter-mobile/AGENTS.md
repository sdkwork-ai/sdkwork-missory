# Repository Guidelines (apps/sdkwork-missory-flutter-mobile)

Read `../../../AGENTS.md` first, then `../../../sdkwork-specs/SOUL.md` and the
task-specific specs under `../../../sdkwork-specs/` (FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md, APP_FLUTTER_UI_SPEC.md, DART_CODE_SPEC.md).

## Local Dictionary

- `lib/bootstrap/`: canonical seams — environment (dart-define), sdk_clients, routes, host_adapters, iam_runtime.
- `packages/sdkwork_missory_flutter_mobile_core/`: generated Dart SDK client + services (only package importing the SDK).
- `env/`: per-profile dart-define documents (`--dart-define-from-file`).
- `sdks/` (repo root): generated Dart SDK `sdkwork_missory_app_sdk`.

## Surface Rules

- Screens consume `MissoryServices` from the bootstrapped runtime; never import the generated SDK directly.
- Verification: `flutter analyze && flutter test` from this directory.
