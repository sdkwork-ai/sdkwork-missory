// H5 environment resolution seam — delegates to the core runtime-config loader.
export { loadMissoryH5RuntimeConfig } from "@sdkwork/missory-h5-core";

import { loadMissoryH5RuntimeConfig } from "@sdkwork/missory-h5-core";

export async function resolveEnvironment() {
  return loadMissoryH5RuntimeConfig();
}
