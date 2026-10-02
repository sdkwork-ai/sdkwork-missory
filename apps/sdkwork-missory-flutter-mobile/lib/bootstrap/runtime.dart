import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

import 'environment.dart';

export 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart'
    show MissoryRuntime;

/// Bootstrap order (FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC section 2):
/// environment -> runtime (SDK clients + services). Host adapters land with
/// the IAM phase-2 adoption.
MissoryRuntime bootstrap() {
  final environment = resolveMissoryEnvironment();
  return MissoryRuntime.bootstrap(environment);
}
