import type { MissoryBriefing } from './missory-briefing';

export interface MissoryBriefingResponse {
  code: 0;
  data: unknown & { item: MissoryBriefing; };
  /** Server-owned request correlation id. */
  traceId: string;
}
