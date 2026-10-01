import type { MissoryTimelineEntry } from './missory-timeline-entry';
import type { PageInfo } from './page-info';

export interface MissoryTimelineEntryPageResponse {
  code: 0;
  data: unknown & { items: MissoryTimelineEntry[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
