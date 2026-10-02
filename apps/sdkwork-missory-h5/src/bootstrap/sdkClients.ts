// H5 SDK client construction seam (clients built only here).
export { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import { createClient } from "@sdkwork/missory-app-sdk";

import type { MissoryH5RuntimeConfig } from "@sdkwork/missory-h5-core";

export function createSdkClients(config: MissoryH5RuntimeConfig) {
  return {
    app: createClient({ baseUrl: config.appApiBaseUrl, platform: "h5" }),
  };
}
