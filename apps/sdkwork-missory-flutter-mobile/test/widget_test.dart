import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';
import 'package:sdkwork_missory_flutter_mobile/app.dart';

void main() {
  test('environment defaults resolve to standalone development', () {
    final environment = MissoryEnvironment.fromDefines();
    expect(environment.profileId, 'standalone.development');
    expect(environment.appApiBaseUrl, 'http://127.0.0.1:8460');
  });

  test('environment rejects unknown profiles', () {
    expect(
      () => MissoryEnvironment.fromDefines(deploymentProfile: 'staging-area'),
      throwsStateError,
    );
  });

  testWidgets('home screen renders the P0 headline', (tester) async {
    SharedPreferences.setMockInitialValues({});
    final environment = MissoryEnvironment.fromDefines();
    final runtime = await MissoryRuntime.bootstrap(environment);
    await tester.pumpWidget(MissoryApp(runtime: runtime));
    await tester.pump(const Duration(milliseconds: 50));
    expect(find.text('今天，有谁值得你想起？'), findsOneWidget);
    expect(find.byType(NavigationBar), findsOneWidget);
  });

  test('session store round-trips the dual-token pair', () {
    const stored = MissoryStoredSession(
      authToken: 'auth-token',
      accessToken: 'access-token',
      refreshToken: 'refresh-token',
      displayName: '李明',
    );
    final decoded = MissoryStoredSession.tryDecode(stored.encode());
    expect(decoded?.authToken, 'auth-token');
    expect(decoded?.accessToken, 'access-token');
    expect(decoded?.refreshToken, 'refresh-token');
    expect(decoded?.displayName, '李明');
    expect(MissoryStoredSession.tryDecode('not-json'), isNull);
    expect(MissoryStoredSession.tryDecode('{"authToken":1}'), isNull);
  });
}
