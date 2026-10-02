import type { MissoryDataExport } from './missory-data-export';

export interface MissoryDataExportResponse {
  code: 0;
  data: unknown & { item: MissoryDataExport; };
  /** Server-owned request correlation id. */
  traceId: string;
}
