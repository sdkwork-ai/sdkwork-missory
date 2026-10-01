import type { MissoryHomeDigest } from './missory-home-digest';

export interface MissoryHomeDigestResponse {
  code: 0;
  data: unknown & { item: MissoryHomeDigest; };
  /** Server-owned request correlation id. */
  traceId: string;
}
