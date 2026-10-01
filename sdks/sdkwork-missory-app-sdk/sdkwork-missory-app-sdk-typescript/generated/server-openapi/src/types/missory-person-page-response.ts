import type { MissoryPerson } from './missory-person';
import type { PageInfo } from './page-info';

export interface MissoryPersonPageResponse {
  code: 0;
  data: unknown & { items: MissoryPerson[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
