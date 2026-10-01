import type { Int64Id } from './int64-id';
import type { MissoryMemoryOrigin } from './missory-memory-origin';

export interface MissoryStory {
  id: Int64Id;
  title: string;
  summary?: string;
  summaryOrigin?: MissoryMemoryOrigin;
  participantIds: Int64Id[];
  memoryIds?: Int64Id[];
  startedAt?: string;
  endedAt?: string;
  location?: string;
  createdAt: string;
  updatedAt: string;
}
