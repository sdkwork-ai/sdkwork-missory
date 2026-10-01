import type { SdkWorkCommandData } from './sdk-work-command-data';

export interface MemoriesRejectResponse {
  code: 0;
  data: SdkWorkCommandData;
  traceId: string;
}
