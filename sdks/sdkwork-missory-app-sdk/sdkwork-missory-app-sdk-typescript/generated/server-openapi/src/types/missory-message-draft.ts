import type { Int64Id } from './int64-id';
import type { MissoryMessageTone } from './missory-message-tone';

export interface MissoryMessageDraft {
  personId: Int64Id;
  draft: string;
  tone: MissoryMessageTone;
  disclaimer?: string;
}
