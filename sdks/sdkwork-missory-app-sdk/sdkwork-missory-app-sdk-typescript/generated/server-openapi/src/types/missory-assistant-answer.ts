import type { MissoryCitation } from './missory-citation';

export interface MissoryAssistantAnswer {
  answer: string;
  citations: MissoryCitation[];
}
