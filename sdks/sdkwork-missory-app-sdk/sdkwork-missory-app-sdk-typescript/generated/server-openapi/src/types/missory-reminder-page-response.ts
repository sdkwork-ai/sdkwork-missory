import type { MissoryReminder } from './missory-reminder';
import type { PageInfo } from './page-info';

export interface MissoryReminderPageResponse {
  code: 0;
  data: unknown & { items: MissoryReminder[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
