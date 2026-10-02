// Environment resolution seam (APP_PC_ARCHITECTURE_SPEC bootstrap layout).
// Delegates to the core runtime-config loader; IAM-aware environment additions
// land with the phase-2 IAM adapter behind the same function signature.
export { loadMissoryPcRuntimeConfig } from "@sdkwork/missory-pc-core";

import { loadMissoryPcRuntimeConfig } from "@sdkwork/missory-pc-core";

export async function resolveEnvironment() {
  return loadMissoryPcRuntimeConfig();
}
