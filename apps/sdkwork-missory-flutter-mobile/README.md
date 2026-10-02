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

## Verify

```bash
flutter analyze   # zero findings expected
flutter test      # environment + home-screen widget tests
```

Release builds (`flutter build apk|appbundle|ipa`) run through the `bin/`
packaging pipeline; release metadata lives in `sdkwork.app.config.json`.
