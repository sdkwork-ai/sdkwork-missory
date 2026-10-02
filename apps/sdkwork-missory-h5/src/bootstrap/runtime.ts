import {
  createMissoryH5Runtime,
  loadMissoryH5RuntimeConfig,
  type MissoryH5Runtime,
} from "@sdkwork/missory-h5-core";

export type { MissoryH5Runtime };

export async function bootstrapMissoryH5Runtime() {
  const config = await loadMissoryH5RuntimeConfig();
  return createMissoryH5Runtime(config);
}

export type BootstrappedMissoryH5Runtime = MissoryH5Runtime;
