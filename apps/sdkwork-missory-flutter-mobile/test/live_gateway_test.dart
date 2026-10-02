// Live-gateway integration test for the Flutter mobile client.
//
// Drives the REAL application core (MissoryRuntime: generated Dart SDK +
// IAM dual-token session + services) against a running standalone gateway:
//
//   SDKWORK_E2E_LIVE=1 flutter test test/live_gateway_test.dart
//
// The gateway must serve the console in the development environment so
// /runtime-env.json carries the deployment bootstrap credential. Without
// SDKWORK_E2E_LIVE the test is a no-op pass, keeping offline CI green.
//
// Chain: bootstrap credential from runtime-env -> registration -> password
// login -> dual-token business calls (my_profile + people) -> whole-account
// privacy export. int64 ids stay strings end to end.
import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

const baseUrl = 'http://127.0.0.1:8464';

void main() {
  test('live gateway: credential-entry login + dual-token business chain', () async {
    if (Platform.environment['SDKWORK_E2E_LIVE'] != '1') {
      // Offline run (default CI): the live chain is covered by the
      // repository-level E2E (`pnpm run test:e2e-login`).
      return;
    }
    TestWidgetsFlutterBinding.ensureInitialized();
    // The test binding routes every HttpClient through an in-memory proxy by
    // default; this test IS the network integration, so restore real sockets.
    HttpOverrides.global = null;
    // The session store persists through shared_preferences; the unit-test VM
    // has no platform channel, so satisfy it with the in-memory shim.
    SharedPreferences.setMockInitialValues({});

    final bootstrap = jsonDecode(await _fetchRuntimeEnv());
    final bootstrapToken = bootstrap['authBootstrapAccessToken'] as String?;
    expect(bootstrapToken, isNotNull, reason: 'gateway must inject the bootstrap credential');
    expect((bootstrapToken as String).split('.').length, 3, reason: 'bootstrap must be a JWT');

    // Register a fresh account through the credential-entry surface.
    final stamp = DateTime.now().millisecondsSinceEpoch;
    final username = 'flutter-e2e-$stamp@missory.test';
    final registration = await _postJson(
      '$baseUrl/app/v3/api/auth/registrations',
      {
        'username': username,
        'password': 'Flutter-Passw0rd!',
        'confirmPassword': 'Flutter-Passw0rd!',
        'displayName': 'Flutter E2E',
      },
      {'Access-Token': bootstrapToken},
    );
    expect(registration['code'], 0, reason: 'registration must succeed');
    expect(registration['data']['user']['id'], isA<String>(),
        reason: 'int64 ids serialize as strings (API_SPEC §13.6)');

    // Drive the real application core: session login + business services.
    final environment = MissoryEnvironment.fromDefines(
      environment: 'development',
      deploymentProfile: 'standalone',
      appApiBaseUrl: baseUrl,
      authBootstrapAccessToken: bootstrapToken,
    );
    final runtime = await MissoryRuntime.bootstrap(environment);
    final session = await runtime.session.loginWithPassword(
      account: username,
      password: 'Flutter-Passw0rd!',
    );
    expect(session.authToken, isNotEmpty);
    expect(runtime.session.isAuthenticated, isTrue);

    final profile = await runtime.services.myProfile();
    expect(profile, isNotNull);
    final userId = profile!.userId;
    expect(userId, isNotNull);
    expect(RegExp(r'^\d+$').hasMatch(userId!), isTrue,
        reason: 'my_profile.userId must be an int64 string');

    final people = await runtime.services.people();
    expect(people, isA<List<dynamic>>());

    final export = await runtime.services.exportData();
    expect(export, isNotNull);
    expect(export!.exportedAt, isNotEmpty);
    expect(export.persons, isA<List<dynamic>>());
    expect(export.memories, isA<List<dynamic>>());
  });
}

Future<String> _fetchRuntimeEnv() async {
  final client = HttpClient();
  try {
    final request = await client.getUrl(Uri.parse('$baseUrl/runtime-env.json'));
    final response = await request.close();
    return await response.transform(utf8.decoder).join();
  } finally {
    client.close(force: true);
  }
}

Future<Map<String, dynamic>> _postJson(
  String url,
  Map<String, dynamic> body,
  Map<String, String> headers,
) async {
  final client = HttpClient();
  try {
    final payload = utf8.encode(jsonEncode(body));
    final request = await client.postUrl(Uri.parse(url));
    request.headers.set('Content-Type', 'application/json');
    headers.forEach(request.headers.set);
    // The gateway's API size policy rejects chunked bodies without a
    // content-length; the app's own transport (package:http) always sends one.
    request.contentLength = payload.length;
    request.add(payload);
    final response = await request.close();
    final text = await response.transform(utf8.decoder).join();
    return jsonDecode(text) as Map<String, dynamic>;
  } finally {
    client.close(force: true);
  }
}
