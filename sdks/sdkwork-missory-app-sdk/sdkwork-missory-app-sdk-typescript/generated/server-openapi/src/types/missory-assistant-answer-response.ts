import type { MissoryAssistantAnswer } from './missory-assistant-answer';

export interface MissoryAssistantAnswerResponse {
  code: 0;
  data: unknown & { item: MissoryAssistantAnswer; };
  /** Server-owned request correlation id. */
  traceId: string;
}
