import type { MissoryImportance } from './missory-importance';
import type { MissoryRelationshipType } from './missory-relationship-type';

export interface MissoryRelationshipUpsertRequest {
  relationshipTypes: MissoryRelationshipType[];
  startedAt?: string;
  lastContactedAt?: string;
  description?: string;
  importance?: MissoryImportance;
  contactCycleDays?: number;
  notes?: string;
}
