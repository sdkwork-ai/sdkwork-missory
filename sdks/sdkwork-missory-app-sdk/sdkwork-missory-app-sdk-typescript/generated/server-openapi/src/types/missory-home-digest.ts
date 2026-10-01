import type { MissoryMemory } from './missory-memory';
import type { MissoryPerson } from './missory-person';
import type { MissoryReminder } from './missory-reminder';

export interface MissoryHomeDigest {
  todayReminders: MissoryReminder[];
  recentMemories: MissoryMemory[];
  recentPersons: MissoryPerson[];
}
