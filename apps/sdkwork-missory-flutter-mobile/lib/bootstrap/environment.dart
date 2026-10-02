import 'package:flutter/foundation.dart';
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

/// Reads the dart-define contract keys from the env profile documents.
MissoryEnvironment resolveMissoryEnvironment() {
  return MissoryEnvironment.fromDefines(
    environment: const String.fromEnvironment('SDKWORK_ENVIRONMENT'),
    deploymentProfile: const String.fromEnvironment('SDKWORK_DEPLOYMENT_PROFILE'),
    profileId: const String.fromEnvironment('SDKWORK_PROFILE_ID'),
    appApiBaseUrl: const String.fromEnvironment(
      'SDKWORK_MISSORY_FLUTTER_APP_API_BASE_URL',
    ),
    authBootstrapAccessToken: const String.fromEnvironment(
      'SDKWORK_MISSORY_AUTH_BOOTSTRAP_ACCESS_TOKEN',
    ),
  );
}

MissoryEnvironment resolveEnvironmentForTest() {
  assert(() {
    debugPrint('missory: resolving environment in test mode');
    return true;
  }());
  return resolveMissoryEnvironment();
}
