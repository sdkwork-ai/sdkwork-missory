import type { MissoryRelationship } from './missory-relationship';

export interface MissoryRelationshipResponse {
  code: 0;
  data: unknown & { item: MissoryRelationship; };
  /** Server-owned request correlation id. */
  traceId: string;
}
