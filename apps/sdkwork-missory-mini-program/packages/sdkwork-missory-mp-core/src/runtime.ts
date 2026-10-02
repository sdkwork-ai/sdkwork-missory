import { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import { createWxFetchAdapter } from "@sdkwork/missory-mp-host";

export interface MissoryMpEnvironment {
  environment: string;
  deploymentProfile: string;
  profileId: string;
  appApiBaseUrl: string;
}

export interface MissoryMpRuntime {
  environment: MissoryMpEnvironment;
  client: SdkworkAppClient;
}

export function resolveMissoryMpEnvironment(input: MissoryMpEnvironment): MissoryMpEnvironment {
  if (!input.appApiBaseUrl.startsWith("http")) {
    throw new Error("missory mp: appApiBaseUrl must be an absolute origin");
  }
  return input;
}

export function bootstrapMissoryMpRuntime(environment: MissoryMpEnvironment): MissoryMpRuntime {
  const wxFetch = createWxFetchAdapter();
  (globalThis as { fetch?: unknown }).fetch = wxFetch as unknown as typeof fetch;
  const client = createClient({ baseUrl: environment.appApiBaseUrl, platform: "mini-program" });
  return { environment, client };
}
