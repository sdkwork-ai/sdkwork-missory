import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { MissoryApi, createMissoryApi } from './api/missory';
import { MissoryAssistantApi, createMissoryAssistantApi } from './api/missory-assistant';

export class SdkworkMissoryAppClient {
  private httpClient: HttpClient;

  public readonly missory: MissoryApi;
  public readonly missoryAssistant: MissoryAssistantApi;

  constructor(config: SdkworkAppConfig) {
    this.httpClient = createHttpClient(config);
    this.missory = createMissoryApi(this.httpClient);

    this.missoryAssistant = createMissoryAssistantApi(this.httpClient);
  }
  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkAppConfig): SdkworkMissoryAppClient {
  return new SdkworkMissoryAppClient(config);
}

export default SdkworkMissoryAppClient;
