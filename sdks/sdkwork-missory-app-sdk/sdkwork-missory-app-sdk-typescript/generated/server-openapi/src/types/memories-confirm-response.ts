import type { SdkWorkCommandData } from './sdk-work-command-data';

export interface MemoriesConfirmResponse {
  code: 0;
  data: SdkWorkCommandData;
  traceId: string;
}
