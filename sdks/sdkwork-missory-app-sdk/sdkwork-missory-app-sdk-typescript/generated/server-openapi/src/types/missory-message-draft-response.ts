import type { MissoryMessageDraft } from './missory-message-draft';

export interface MissoryMessageDraftResponse {
  code: 0;
  data: unknown & { item: MissoryMessageDraft; };
  /** Server-owned request correlation id. */
  traceId: string;
}
