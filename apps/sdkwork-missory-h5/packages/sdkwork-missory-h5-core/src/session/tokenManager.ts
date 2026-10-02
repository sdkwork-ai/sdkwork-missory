// Session/token wiring for the H5 console (see pc-core tokenManager for the
// fleet contract: dev seeds the bypass identity; IAM login sets real tokens).
import { createTokenManager, type AuthTokenManager } from "@sdkwork/sdk-common";

import type { MissoryH5RuntimeConfig } from "../config/runtime-config.ts";

export function createTokenManagerFor(config: MissoryH5RuntimeConfig): AuthTokenManager {
  const tokenManager = createTokenManager();
  if (config.environment === "development") {
    tokenManager.setAuthToken("dev-auth-token");
    tokenManager.setAccessToken("dev-access-token");
  }
  return tokenManager;
}
