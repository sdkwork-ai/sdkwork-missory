import type { Int64Id } from './int64-id';

export interface MissoryMemoryExtractRequest {
  /** Optional target person; omitted resolves by name mentions in text. */
  personId?: Int64Id;
  text: string;
}
