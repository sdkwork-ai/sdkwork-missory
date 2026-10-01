import type { MissoryMemory } from './missory-memory';

export interface MissoryMemoryResponse {
  code: 0;
  data: unknown & { item: MissoryMemory; };
  /** Server-owned request correlation id. */
  traceId: string;
}
