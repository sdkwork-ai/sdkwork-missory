import type { Int64Id } from './int64-id';
import type { MissoryMemory } from './missory-memory';

export interface MissoryChatSummary {
  personId: Int64Id;
  summary: string;
  candidateMemories: MissoryMemory[];
}
