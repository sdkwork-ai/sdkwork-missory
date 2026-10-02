import { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import { createWxFetchAdapter } from "@sdkwork/missory-mp-host";

import {
  createMissoryMpSessionFacade,
  createMissoryMpTokenManager,
  type MissoryMpSessionFacade,
} from "./session.ts";

export interface MissoryMpEnvironment {
  environment: string;
  deploymentProfile: string;
  profileId: string;
  appApiBaseUrl: string;
  /**
   * Deployment-provisioned credential-entry bootstrap `Access-Token`
   * (IAM login/registration tenant isolation). Stamped in by the runtime
   * bundle build from the optional runtime-env key
   * `SDKWORK_MISSORY_AUTH_BOOTSTRAP_ACCESS_TOKEN`; development with the IAM
   * dev authentication fallback accepts any value.
   */
  authBootstrapAccessToken?: string;
}

export interface MissoryMpRuntime {
  environment: MissoryMpEnvironment;
  client: SdkworkAppClient;
  session: MissoryMpSessionFacade;
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
  // IAM dual-token runtime: the token manager carries the stored login pair
  // (or the development bypass identity) and every request dispatches
  // Authorization/Access-Token through it (TECH_ARCHITECTURE section 8).
  const tokenManager = createMissoryMpTokenManager(environment);
  const client = createClient({
    baseUrl: environment.appApiBaseUrl,
    platform: "mini-program",
    authMode: "dual-token",
    tokenManager,
  });
  return {
    environment,
    client,
    session: createMissoryMpSessionFacade(environment, tokenManager),
  };
}
