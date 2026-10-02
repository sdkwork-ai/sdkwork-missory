# sdkwork-missory-flutter-mobile

Flutter mobile app for 念忆 · Missory (AI personal social memory): home digest,
people, memories (Fact ≠ Inference confirm workflow), and the draft-only AI
assistant.

## Run against the local gateway

```bash
flutter pub get
flutter run --dart-define-from-file=env/sdkwork.standalone.development.json
```

`env/sdkwork.<profile>.<environment>.json` declares the dart-define contract
(`SDKWORK_ENVIRONMENT`, `SDKWORK_PROFILE_ID`, app API base URL — Android
emulator loopback uses `10.0.2.2`).

### Credential-entry bootstrap token (real-gateway development runs)

The IAM credential-entry surface requires the deployment-provisioned
bootstrap `Access-Token` JWT. The standalone gateway injects it into the
same-origin browser consoles automatically; the Flutter client receives it
through the env profile instead. Issue one from the built gateway and put it
into the profile you run with:

```bash
# from the repository root, with the dev database configured
./target/debug/sdkwork-api-missory-standalone-gateway.exe issue-bootstrap-token --tenant 100001 --app sdkwork-missory
# then set SDKWORK_MISSORY_AUTH_BOOTSTRAP_ACCESS_TOKEN in
# env/sdkwork.<profile>.<environment>.json to the issued accessCredential
```

Without it, development rides the well-known dev identity only when the
gateway runs with `SDKWORK_MISSORY_DEV_AUTH_BYPASS=true` (API-level smoke
topology; the credential-entry login surface always needs a real bootstrap
JWT).

## Live-gateway integration test

```bash
SDKWORK_E2E_LIVE=1 SDKWORK_E2E_BASE_URL=http://127.0.0.1:8464 flutter test test/live_gateway_test.dart
```

Drives the real application core (registration -> password login ->
my_profile -> people -> data export) against the running gateway; without
`SDKWORK_E2E_LIVE` the test is a no-op so offline CI stays green.

## Verify

```bash
flutter analyze   # zero findings expected
flutter test      # environment + home-screen widget tests
```

Release builds (`flutter build apk|appbundle|ipa`) run through the `bin/`
packaging pipeline; release metadata lives in `sdkwork.app.config.json`.
