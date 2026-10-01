import type { Int64Id } from './int64-id';
import type { MissoryImportance } from './missory-importance';
import type { MissoryMemoryType } from './missory-memory-type';
import type { MissorySourceKind } from './missory-source-kind';

export interface MissoryMemoryUpsertRequest {
  personId: Int64Id;
  storyId?: Int64Id;
  type: MissoryMemoryType;
  title?: string;
  content: string;
  importance?: MissoryImportance;
  occurredAt?: string;
  sourceKind?: MissorySourceKind;
  sourceRef?: string;
}
