import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

import 'environment.dart';

/// SDK client construction seam: the generated Dart app-sdk client is created
/// only here (FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC section 2 bootstrap order).
MissoryRuntime createSdkClients() {
  final environment = resolveMissoryEnvironment();
  return MissoryRuntime.bootstrap(environment);
}
