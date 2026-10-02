// Session/token wiring for the PC console.
//
// Fleet contract: app-api is a dual-token surface, so the generated client
// requires an Access-Token via the shared token manager before dispatch. The
// IAM login flow (phase 2) sets real tokens; standalone development seeds the
// well-known dev identity accepted by the gateway's DEV_AUTH_BYPASS —
// production environments refuse that bypass (ENVIRONMENT_SPEC section 7.2).
import { createTokenManager, type AuthTokenManager } from "@sdkwork/sdk-common";

import type { MissoryPcRuntimeConfig } from "../config/runtime-config.ts";

export function createTokenManagerFor(config: MissoryPcRuntimeConfig): AuthTokenManager {
  const tokenManager = createTokenManager();
  if (config.environment === "development") {
    tokenManager.setAuthToken("dev-auth-token");
    tokenManager.setAccessToken("dev-access-token");
  }
  return tokenManager;
}
