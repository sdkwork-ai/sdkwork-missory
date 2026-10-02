import type { MissoryMemory, MissoryMemoryExtractRequest, MissoryMemoryUpsertRequest, PageInfo, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export interface MemoriesListQuery {
  page?: number;
  pageSize?: number;
  q?: string;
  personId?: string;
  status?: string;
  origin?: string;
  type?: string;
}

export interface MemoriesPage {
  items: MissoryMemory[];
  pageInfo: PageInfo;
}

export function createMemoriesService(client: SdkworkAppClient) {
  return {
    list(query: MemoriesListQuery = {}): Promise<MemoriesPage> {
      return client.missory.memories.list({
        page: query.page,
        pageSize: query.pageSize,
        q: query.q,
        personId: query.personId,
        status: query.status as never,
        origin: query.origin as never,
        type_: query.type as never,
      });
    },
    retrieve(memoryId: string) {
      return client.missory.memories.retrieve(memoryId);
    },
    create(request: MissoryMemoryUpsertRequest) {
      return client.missory.memories.create(request);
    },
    update(memoryId: string, request: MissoryMemoryUpsertRequest) {
      return client.missory.memories.update(memoryId, request);
    },
    async delete(memoryId: string): Promise<void> {
      await client.missory.memories.delete(memoryId);
    },
    extract(request: MissoryMemoryExtractRequest) {
      return client.missory.memories.extract(request);
    },
    confirm(memoryId: string) {
      return client.missory.memories.confirm(memoryId);
    },
    reject(memoryId: string) {
      return client.missory.memories.reject(memoryId);
    },
  };
}
