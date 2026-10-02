// SDK client construction seam. The generated app-sdk client is created only
// here (APP_SDK_INTEGRATION_SPEC section 9); screens receive it via the
// bootstrapped runtime.
export { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import { createClient } from "@sdkwork/missory-app-sdk";

import type { MissoryPcRuntimeConfig } from "@sdkwork/missory-pc-core";

export function createSdkClients(config: MissoryPcRuntimeConfig) {
  return {
    app: createClient({ baseUrl: config.appApiBaseUrl, platform: "pc" }),
  };
}
