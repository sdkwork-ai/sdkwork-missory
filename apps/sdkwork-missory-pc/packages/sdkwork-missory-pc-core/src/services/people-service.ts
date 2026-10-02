import type { MissoryPerson, MissoryPersonUpsertRequest, MissoryRelationship, MissoryRelationshipUpsertRequest, MissoryTimelineEntry, PageInfo, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export interface PeopleListQuery {
  page?: number;
  pageSize?: number;
  q?: string;
}

export interface PeoplePage {
  items: MissoryPerson[];
  pageInfo: PageInfo;
}

export function createPeopleService(client: SdkworkAppClient) {
  return {
    async list(query: PeopleListQuery = {}): Promise<PeoplePage> {
      return client.missory.persons.list({
        page: query.page,
        pageSize: query.pageSize,
        q: query.q,
      });
    },
    retrieve(personId: string) {
      return client.missory.persons.retrieve(personId);
    },
    create(request: MissoryPersonUpsertRequest) {
      return client.missory.persons.create(request);
    },
    update(personId: string, request: MissoryPersonUpsertRequest) {
      return client.missory.persons.update(personId, request);
    },
    async delete(personId: string): Promise<void> {
      await client.missory.persons.delete(personId);
    },
    async timeline(personId: string): Promise<MissoryTimelineEntry[]> {
      const page = await client.missory.persons.timeline.list(personId);
      return page.items;
    },
    upsertRelationship(personId: string, request: MissoryRelationshipUpsertRequest): Promise<MissoryRelationship> {
      return client.missory.relationships.create(personId, request);
    },
    deleteRelationship(personId: string, relationshipId: string): Promise<void> {
      return client.missory.relationships.delete(personId, relationshipId);
    },
  };
}
