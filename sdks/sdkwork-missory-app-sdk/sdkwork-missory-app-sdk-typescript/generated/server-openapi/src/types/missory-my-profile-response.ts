import type { MissoryMyProfile } from './missory-my-profile';

export interface MissoryMyProfileResponse {
  code: 0;
  data: unknown & { item: MissoryMyProfile; };
  /** Server-owned request correlation id. */
  traceId: string;
}
