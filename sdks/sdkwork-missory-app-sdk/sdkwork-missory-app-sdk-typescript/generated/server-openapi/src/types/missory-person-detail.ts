import type { MissoryMemory } from './missory-memory';
import type { MissoryPerson } from './missory-person';
import type { MissoryRelationship } from './missory-relationship';
import type { MissoryStory } from './missory-story';

export interface MissoryPersonDetail {
  person: MissoryPerson;
  relationships: MissoryRelationship[];
  recentMemories: MissoryMemory[];
  commitments: MissoryMemory[];
  stories?: MissoryStory[];
  stats: { memoryCount: number; storyCount: number; knownDays: number; daysSinceLastContact?: number; };
}
