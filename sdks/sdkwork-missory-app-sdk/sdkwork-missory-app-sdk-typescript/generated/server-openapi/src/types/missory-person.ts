import type { Int64Id } from './int64-id';

export interface MissoryPerson {
  id: Int64Id;
  displayName: string;
  aliases?: string[];
  gender?: string;
  /** MM-DD or full date; drives birthday reminders. */
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
  lastContactedAt?: string;
  createdAt: string;
  updatedAt: string;
}
