import type { Int64Id } from './int64-id';

export interface MissoryBriefing {
  personId: Int64Id;
  briefing: string;
  topics?: string[];
}
