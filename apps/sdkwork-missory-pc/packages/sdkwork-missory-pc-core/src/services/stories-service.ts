import type { MissoryStoryUpsertRequest, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createStoriesService(client: SdkworkAppClient) {
  return {
    list(params: { page?: number; pageSize?: number; q?: string } = {}) {
      return client.missory.stories.list({
        page: params.page,
        pageSize: params.pageSize,
        q: params.q,
      });
    },
    retrieve(storyId: string) {
      return client.missory.stories.retrieve(storyId);
    },
    create(request: MissoryStoryUpsertRequest) {
      return client.missory.stories.create(request);
    },
    update(storyId: string, request: MissoryStoryUpsertRequest) {
      return client.missory.stories.update(storyId, request);
    },
    async delete(storyId: string): Promise<void> {
      await client.missory.stories.delete(storyId);
    },
    summarize(storyId: string) {
      return client.missoryAssistant.stories.summaries.create(storyId);
    },
  };
}
