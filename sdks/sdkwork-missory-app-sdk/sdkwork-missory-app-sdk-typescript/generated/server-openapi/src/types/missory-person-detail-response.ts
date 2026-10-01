import type { MissoryPersonDetail } from './missory-person-detail';

export interface MissoryPersonDetailResponse {
  code: 0;
  data: unknown & { item: MissoryPersonDetail; };
  /** Server-owned request correlation id. */
  traceId: string;
}
