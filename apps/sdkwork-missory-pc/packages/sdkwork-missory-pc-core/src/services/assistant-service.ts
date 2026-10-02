import type { SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createAssistantService(client: SdkworkAppClient) {
  return {
    query(question: string) {
      return client.missoryAssistant.assistant.query({ question });
    },
    briefing(personId: string) {
      return client.missoryAssistant.assistant.briefings.create({ personId });
    },
    messageDraft(personId: string, scenario: string, tone?: string, note?: string) {
      return client.missoryAssistant.assistant.messageDrafts.create({
        personId,
        scenario: scenario as never,
        ...(tone ? { tone: tone as never } : {}),
        ...(note ? { note } : {}),
      });
    },
    chatSummary(personId: string, text: string) {
      return client.missoryAssistant.assistant.chatSummaries.create({ personId, text });
    },
    storySummary(storyId: string) {
      return client.missoryAssistant.stories.summaries.create(storyId);
    },
  };
}
