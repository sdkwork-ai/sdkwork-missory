import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;

import 'environment.dart';
import 'services.dart';

/// Bootstrapped Missory runtime: the generated app-sdk client plus services.
///
/// IAM dual-token wiring is the phase-2 adoption; standalone development
/// gateways accept the dev identity bypass (see TECH ARCHITECTURE section 8).
class MissoryRuntime {
  MissoryRuntime._({required this.environment, required this.client});

  final MissoryEnvironment environment;
  final sdk.SdkworkMissoryAppClient client;

  late final MissoryServices services = MissoryServices(client);

  /// Bootstraps the runtime for [environment].
  factory MissoryRuntime.bootstrap(MissoryEnvironment environment) {
    final client = sdk.SdkworkMissoryAppClient.withBaseUrl(
      baseUrl: environment.appApiBaseUrl,
    );
    return MissoryRuntime._(environment: environment, client: client);
  }
}
