import type { MissoryPerson } from './missory-person';

export interface MissoryPersonResponse {
  code: 0;
  data: unknown & { item: MissoryPerson; };
  /** Server-owned request correlation id. */
  traceId: string;
}
