import type { Int64Id } from './int64-id';
import type { MissoryImportance } from './missory-importance';
import type { MissoryRelationshipType } from './missory-relationship-type';

export interface MissoryRelationship {
  id: Int64Id;
  personId: Int64Id;
  relationshipTypes: MissoryRelationshipType[];
  startedAt?: string;
  lastContactedAt?: string;
  description?: string;
  importance?: MissoryImportance;
  /** Custom long-uncontacted cycle in days. */
  contactCycleDays?: number;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}
