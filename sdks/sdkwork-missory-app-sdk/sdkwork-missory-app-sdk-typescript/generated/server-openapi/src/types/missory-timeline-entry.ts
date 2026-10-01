import type { Int64Id } from './int64-id';

export interface MissoryTimelineEntry {
  occurredAt: string;
  kind: 'relationship-start' | 'memory' | 'story' | 'contact';
  title: string;
  detail?: string;
  memoryId?: Int64Id;
  storyId?: Int64Id;
}
