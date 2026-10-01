import type { Int64Id } from './int64-id';
import type { MissoryImportance } from './missory-importance';
import type { MissoryMemoryOrigin } from './missory-memory-origin';
import type { MissoryMemoryStatus } from './missory-memory-status';
import type { MissoryMemoryType } from './missory-memory-type';
import type { MissorySourceKind } from './missory-source-kind';

export interface MissoryMemory {
  id: Int64Id;
  personId: Int64Id;
  storyId?: Int64Id;
  type: MissoryMemoryType;
  title?: string;
  content: string;
  origin: MissoryMemoryOrigin;
  status: MissoryMemoryStatus;
  /** Inference-only confidence in [0,1]. */
  confidence?: number;
  /** Inference-only reasoning note. */
  sourceReason?: string;
  sourceKind: MissorySourceKind;
  sourceRef?: string;
  importance?: MissoryImportance;
  occurredAt?: string;
  createdAt: string;
  updatedAt: string;
}
