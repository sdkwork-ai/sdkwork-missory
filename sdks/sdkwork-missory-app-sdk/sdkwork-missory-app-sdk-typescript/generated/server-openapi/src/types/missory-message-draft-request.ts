import type { Int64Id } from './int64-id';
import type { MissoryAssistantScenario } from './missory-assistant-scenario';
import type { MissoryMessageTone } from './missory-message-tone';

export interface MissoryMessageDraftRequest {
  personId: Int64Id;
  scenario: MissoryAssistantScenario;
  tone?: MissoryMessageTone;
  note?: string;
}
