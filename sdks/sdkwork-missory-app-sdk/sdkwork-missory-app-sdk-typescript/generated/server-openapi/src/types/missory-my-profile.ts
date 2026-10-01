import type { Int64Id } from './int64-id';

export interface MissoryMyProfile {
  userId: Int64Id;
  displayName: string;
  nickname?: string;
  city?: string;
  occupation?: string;
  company?: string;
  education?: string;
  interests?: string[];
  likes?: string[];
  dislikes?: string[];
  communicationStyle?: string;
  bio?: string;
  createdAt?: string;
  updatedAt: string;
}
