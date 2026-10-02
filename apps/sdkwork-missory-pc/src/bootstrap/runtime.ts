import {
  createMissoryPcRuntime,
  loadMissoryPcRuntimeConfig,
  type MissoryPcRuntime,
} from "@sdkwork/missory-pc-core";

export type { MissoryPcRuntime };

export async function bootstrapMissoryPcRuntime() {
  const config = await loadMissoryPcRuntimeConfig();
  return createMissoryPcRuntime(config);
}

export type BootstrappedMissoryPcRuntime = MissoryPcRuntime;
