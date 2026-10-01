import type { MissoryStory } from './missory-story';

export interface MissoryStoryResponse {
  code: 0;
  data: unknown & { item: MissoryStory; };
  /** Server-owned request correlation id. */
  traceId: string;
}
