import type { Int64Id } from './int64-id';

export interface MissoryStoryUpsertRequest {
  title: string;
  participantIds?: Int64Id[];
  memoryIds?: Int64Id[];
  startedAt?: string;
  endedAt?: string;
  location?: string;
}
