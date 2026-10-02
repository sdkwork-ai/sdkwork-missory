// IAM credential-entry session for the Flutter mobile app (mirrors the
// pc-core/h5-core/mp-core session facades).
//
// `POST /app/v3/api/auth/sessions` is a credential-entry route: it requires
// only the deployment-provisioned bootstrap `Access-Token` (tenant isolation;
// no Authorization header) and returns the dual-token pair every app-api
// request then carries: `Authorization: Bearer <authToken>` +
// `Access-Token: <accessToken>`. The pair persists in shared_preferences so a
// cold start stays signed in. Development without a provisioned bootstrap
// token rides the gateway's IAM dev authentication fallback; non-development
// deployments inject the bootstrap token through the dart-define env profile.
library;

import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;

import 'environment.dart';

/// Stored dual-token session (persisted in shared_preferences).
class MissoryStoredSession {
  const MissoryStoredSession({
    required this.authToken,
    required this.accessToken,
    this.refreshToken,
    this.displayName,
  });

  final String authToken;
  final String accessToken;
  final String? refreshToken;
  final String? displayName;

  static MissoryStoredSession? tryDecode(String? raw) {
    if (raw == null || raw.isEmpty) return null;
    try {
      final decoded = jsonDecode(raw);
      if (decoded is! Map<String, dynamic>) return null;
      final authToken = decoded['authToken'];
      final accessToken = decoded['accessToken'];
      if (authToken is! String || accessToken is! String) return null;
      return MissoryStoredSession(
        authToken: authToken,
        accessToken: accessToken,
        refreshToken: decoded['refreshToken'] is String ? decoded['refreshToken'] as String : null,
        displayName: decoded['displayName'] is String ? decoded['displayName'] as String : null,
      );
    } on FormatException {
      return null;
    }
  }

  String encode() => jsonEncode({
        'authToken': authToken,
        'accessToken': accessToken,
        if (refreshToken != null) 'refreshToken': refreshToken,
        if (displayName != null) 'displayName': displayName,
      });
}

/// Login failure carrying the problem detail (`application/problem+json`) or a
/// transport message.
class MissoryLoginException implements Exception {
  MissoryLoginException(this.message, {this.code, this.status = 0});

  final String message;
  final int? code;
  final int status;

  @override
  String toString() => message;
}

/// Session facade: restore/login/logout plus projection into the generated
/// SDK client (`setAuthToken`/`setAccessToken`).
class MissorySession {
  MissorySession._({
    required this.environment,
    required this.tokenManagerClient,
    required this.session,
  });

  final MissoryEnvironment environment;

  /// Token holder used for header projection (the app's generated client).
  final sdk.SdkworkMissoryAppClient tokenManagerClient;

  MissoryStoredSession? session;

  static const _storageKey = 'sdkwork.missory.flutter.session';

  /// Restores the persisted session (if any) and applies it to [client].
  static Future<MissorySession> load({
    required MissoryEnvironment environment,
    required sdk.SdkworkMissoryAppClient client,
  }) async {
    final prefs = await SharedPreferences.getInstance();
    final session = MissoryStoredSession.tryDecode(prefs.getString(_storageKey));
    final facade = MissorySession._(
      environment: environment,
      tokenManagerClient: client,
      session: session,
    );
    if (session != null) {
      facade._apply(session);
    } else if (environment.environment == 'development') {
      // Standalone development gateway (DEV_AUTH_BYPASS) accepts the
      // well-known dev identity; production refuses that bypass.
      client.setAuthToken('dev-auth-token');
      client.setAccessToken('dev-access-token');
    }
    return facade;
  }

  bool get isAuthenticated => session != null;

  /// Credential-entry password login against the same-origin IAM app-api.
  Future<MissoryStoredSession> loginWithPassword({
    required String account,
    required String password,
  }) async {
    final trimmed = account.trim();
    if (trimmed.isEmpty || password.isEmpty) {
      throw MissoryLoginException('请输入账号和密码');
    }
    final bootstrap = environment.authBootstrapAccessToken.isNotEmpty
        ? environment.authBootstrapAccessToken
        : (environment.environment == 'development' ? 'dev-bootstrap-access-token' : null);
    if (bootstrap == null) {
      throw MissoryLoginException(
        '缺少部署预置的凭证入口 Access-Token（runtime-env authBootstrapAccessToken）',
      );
    }
    final uri = Uri.parse('${environment.appApiBaseUrl}/app/v3/api/auth/sessions');
    final http.Response response;
    try {
      response = await http
          .post(
            uri,
            headers: {
              'Content-Type': 'application/json',
              'Access-Token': bootstrap,
            },
            body: jsonEncode({'grantType': 'password', 'username': trimmed, 'password': password}),
          )
          .timeout(const Duration(seconds: 15));
    } catch (error) {
      throw MissoryLoginException('无法连接登录服务：$error');
    }
    if (response.statusCode != 200) {
      throw _problemFrom(response.statusCode, response.body);
    }
    final envelope = jsonDecode(response.body);
    if (envelope is! Map<String, dynamic> ||
        envelope['code'] != 0 ||
        envelope['data'] is! Map<String, dynamic>) {
      throw MissoryLoginException('登录响应状态异常', status: 200);
    }
    final data = envelope['data'] as Map<String, dynamic>;
    final authToken = data['authToken'];
    final accessToken = data['accessToken'];
    if (authToken is! String || accessToken is! String) {
      throw MissoryLoginException('登录响应缺少双令牌', status: 200);
    }
    final user = data['user'];
    final stored = MissoryStoredSession(
      authToken: authToken,
      accessToken: accessToken,
      refreshToken: data['refreshToken'] is String ? data['refreshToken'] as String : null,
      displayName:
          user is Map<String, dynamic> && user['displayName'] is String ? user['displayName'] as String : null,
    );
    await _persist(stored);
    _apply(stored);
    // Track the in-memory pair: isAuthenticated reads this field, and the
    // gate must be signed-in immediately after a successful login (a cold
    // start re-loads it from persistence).
    session = stored;
    return stored;
  }

  /// Clears the persisted and in-memory pair.
  Future<void> logout() async {
    session = null;
    tokenManagerClient.setAuthToken('');
    tokenManagerClient.setAccessToken('');
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_storageKey);
  }

  void _apply(MissoryStoredSession stored) {
    tokenManagerClient.setAuthToken(stored.authToken);
    tokenManagerClient.setAccessToken(stored.accessToken);
  }

  Future<void> _persist(MissoryStoredSession stored) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_storageKey, stored.encode());
  }

  MissoryLoginException _problemFrom(int status, String body) {
    try {
      final decoded = jsonDecode(body);
      if (decoded is Map<String, dynamic> && decoded['detail'] is String) {
        return MissoryLoginException(
          decoded['detail'] as String,
          code: decoded['code'] is int ? decoded['code'] as int : null,
          status: status,
        );
      }
    } on FormatException {
      // fall through to the generic message
    }
    return MissoryLoginException('登录失败（HTTP $status）', status: status);
  }
}
