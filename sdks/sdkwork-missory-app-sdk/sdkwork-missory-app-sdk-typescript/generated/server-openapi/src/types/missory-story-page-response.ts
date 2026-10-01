import type { MissoryStory } from './missory-story';
import type { PageInfo } from './page-info';

export interface MissoryStoryPageResponse {
  code: 0;
  data: unknown & { items: MissoryStory[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
