import type { MissoryChatSummary } from './missory-chat-summary';

export interface MissoryChatSummaryResponse {
  code: 0;
  data: unknown & { item: MissoryChatSummary; };
  /** Server-owned request correlation id. */
  traceId: string;
}
