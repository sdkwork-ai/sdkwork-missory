import 'src/http/client.dart';
import 'src/http/sdk_config.dart';
import 'src/api/missory.dart';
import 'src/api/missory_assistant.dart';

class SdkworkMissoryAppClient {
  final HttpClient _httpClient;

  late final MissoryApi missory;
  late final MissoryAssistantApi missoryAssistant;

  SdkworkMissoryAppClient({
    required SdkConfig config,
  }) : _httpClient = HttpClient(config: config) {
    missory = MissoryApi(_httpClient);
    missoryAssistant = MissoryAssistantApi(_httpClient);
  }

  factory SdkworkMissoryAppClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkMissoryAppClient(
      config: SdkConfig(
        baseUrl: baseUrl,
        timeout: timeout,
        headers: headers ?? const {},
        authToken: authToken,
        accessToken: accessToken,
      ),
    );
  }

  void setAuthToken(String token) {
    _httpClient.setAuthToken(token);
  }

  void setAccessToken(String token) {
    _httpClient.setAccessToken(token);
  }

  void setHeader(String key, String value) {
    _httpClient.setHeader(key, value);
  }

  void close() {
    _httpClient.close();
  }
}
