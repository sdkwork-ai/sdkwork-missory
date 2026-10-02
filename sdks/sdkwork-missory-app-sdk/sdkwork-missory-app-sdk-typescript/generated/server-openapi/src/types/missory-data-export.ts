import type { MissoryMemory } from './missory-memory';
import type { MissoryMyProfile } from './missory-my-profile';
import type { MissoryPerson } from './missory-person';
import type { MissoryRelationship } from './missory-relationship';
import type { MissoryReminder } from './missory-reminder';
import type { MissoryStory } from './missory-story';

export interface MissoryDataExport {
  exportedAt: string;
  profile: MissoryMyProfile;
  persons: MissoryPerson[];
  relationships: MissoryRelationship[];
  memories: MissoryMemory[];
  stories: MissoryStory[];
  reminders: MissoryReminder[];
}
