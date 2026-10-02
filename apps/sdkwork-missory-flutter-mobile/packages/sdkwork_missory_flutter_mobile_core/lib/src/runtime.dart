import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;

import 'environment.dart';
import 'services.dart';
import 'session.dart';

/// Bootstrapped Missory runtime: the generated app-sdk client, the IAM
/// dual-token session, and services.
///
/// The session restores the persisted token pair (or seeds the well-known dev
/// identity under the development gateway bypass) and projects it into the
/// client before the first dispatch (TECH ARCHITECTURE section 8).
class MissoryRuntime {
  MissoryRuntime._({
    required this.environment,
    required this.client,
    required this.session,
  });

  final MissoryEnvironment environment;
  final sdk.SdkworkMissoryAppClient client;
  final MissorySession session;

  late final MissoryServices services = MissoryServices(client);

  /// Bootstraps the runtime for [environment].
  static Future<MissyRuntime> bootstrap(MissyEnvironment environment) async {
    final client = sdk.SdkworkMissoryAppClient.withBaseUrl(
      baseUrl: environment.appApiBaseUrl,
    );
    final session = await MissorySession.load(environment: environment, client: client);
    return MissoryRuntime._(environment: environment, client: client, session: session);
  }
}
