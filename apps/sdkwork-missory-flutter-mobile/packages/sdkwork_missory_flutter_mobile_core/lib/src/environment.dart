// Runtime environment resolved from --dart-define-from-file values
// declared by the env profile documents (see env/README.md).
class MissoryEnvironment {
  const MissoryEnvironment({
    required this.environment,
    required this.deploymentProfile,
    required this.profileId,
    required this.appApiBaseUrl,
    this.authBootstrapAccessToken = '',
  });

  final String environment;
  final String deploymentProfile;
  final String profileId;
  final String appApiBaseUrl;

  /// Deployment-provisioned credential-entry bootstrap `Access-Token` (IAM
  /// login/registration tenant isolation); empty in development profiles where
  /// the gateway IAM dev authentication fallback accepts any value.
  final String authBootstrapAccessToken;

  static const _knownEnvironments = {'development', 'test', 'staging', 'demo', 'production'};
  static const _knownProfiles = {'standalone', 'cloud'};

  /// Builds the environment from dart-define strings with local defaults.
  factory MissoryEnvironment.fromDefines({
    String environment = '',
    String deploymentProfile = '',
    String profileId = '',
    String appApiBaseUrl = '',
    String authBootstrapAccessToken = '',
  }) {
    final resolvedEnvironment = environment.isNotEmpty ? environment : 'development';
    final resolvedProfile = deploymentProfile.isNotEmpty ? deploymentProfile : 'standalone';
    if (!_knownEnvironments.contains(resolvedEnvironment)) {
      throw StateError('missory: unknown environment "$resolvedEnvironment"');
    }
    if (!_knownProfiles.contains(resolvedProfile)) {
      throw StateError('missory: unknown deployment profile "$resolvedProfile"');
    }
    final resolvedBase = appApiBaseUrl.isNotEmpty ? appApiBaseUrl : 'http://127.0.0.1:8460';
    final uri = Uri.tryParse(resolvedBase);
    if (uri == null || !uri.hasScheme) {
      throw StateError('missory: appApiBaseUrl must be an absolute URL');
    }
    return MissoryEnvironment(
      environment: resolvedEnvironment,
      deploymentProfile: resolvedProfile,
      profileId:
          profileId.isNotEmpty ? profileId : '$resolvedProfile.$resolvedEnvironment',
      appApiBaseUrl: resolvedBase.replaceAll(RegExp(r'/+$'), ''),
      authBootstrapAccessToken: authBootstrapAccessToken,
    );
  }
}
