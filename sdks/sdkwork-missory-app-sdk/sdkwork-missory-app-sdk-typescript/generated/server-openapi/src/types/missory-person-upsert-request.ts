import type { MissoryRelationshipType } from './missory-relationship-type';

export interface MissoryPersonUpsertRequest {
  displayName: string;
  aliases?: string[];
  gender?: string;
  birthday?: string;
  city?: string;
  title?: string;
  company?: string;
  avatarUrl?: string;
  tags?: string[];
  interests?: string[];
  preferences?: string[];
  bio?: string;
  contactChannels?: Record<string, string>;
  notes?: string;
  /** Optional initial relationship types on create. */
  relationshipTypes?: MissoryRelationshipType[];
}
