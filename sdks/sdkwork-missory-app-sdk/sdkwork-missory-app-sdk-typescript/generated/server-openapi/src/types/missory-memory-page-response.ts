import type { MissoryMemory } from './missory-memory';
import type { PageInfo } from './page-info';

export interface MissoryMemoryPageResponse {
  code: 0;
  data: unknown & { items: MissoryMemory[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
