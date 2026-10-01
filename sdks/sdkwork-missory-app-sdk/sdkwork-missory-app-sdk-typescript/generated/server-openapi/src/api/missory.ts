import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { Int64Id, MissoryHomeDigest, MissoryMemory, MissoryMemoryExtractRequest, MissoryMemoryOrigin, MissoryMemoryStatus, MissoryMemoryType, MissoryMemoryUpsertRequest, MissoryMyProfile, MissoryMyProfileUpsertRequest, MissoryPerson, MissoryPersonDetail, MissoryPersonUpsertRequest, MissoryRelationship, MissoryRelationshipType, MissoryRelationshipUpsertRequest, MissoryReminder, MissoryReminderSnoozeRequest, MissoryReminderType, MissoryStory, MissoryStoryUpsertRequest, MissoryTimelineEntry, PageInfo, SdkWorkCommandData } from '../types';


export class MissoryHomeTodayApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Home digest — today relations, recent memories, recent persons. */
  async list(requestOptions?: ApiRequestOptions): Promise<MissoryHomeDigest> {
    return this.client.request<MissoryHomeDigest>(appApiPath(`/app/v3/api/missory/home/today`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryHomeApi {
  public readonly today: MissoryHomeTodayApi;

  constructor(client: HttpClient) {
    this.today = new MissoryHomeTodayApi(client);
  }

}

export interface MissoryRemindersListParams {
  page?: number;
  pageSize?: number;
  type_?: MissoryReminderType;
}

export class MissoryRemindersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List derived reminders (long-uncontacted, birthday, commitment, events). */
  async list(params?: MissoryRemindersListParams, requestOptions?: ApiRequestOptions): Promise<{ items: MissoryReminder[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'type', value: params?.type_, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MissoryReminder[]; pageInfo: PageInfo; }>(appendQueryString(appApiPath(`/app/v3/api/missory/reminders`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Dismiss a derived reminder. */
  async dismiss(reminderId: string, requestOptions?: ApiRequestOptions): Promise<SdkWorkCommandData> {
    return this.client.request<SdkWorkCommandData>(appApiPath(`/app/v3/api/missory/reminders/${serializePathParameter(reminderId, { name: 'reminderId', style: 'simple', explode: false })}/dismiss`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'command' });
  }

/** Snooze a derived reminder for a number of days. */
  async snooze(reminderId: string, body: MissoryReminderSnoozeRequest, requestOptions?: ApiRequestOptions): Promise<SdkWorkCommandData> {
    return this.client.request<SdkWorkCommandData>(appApiPath(`/app/v3/api/missory/reminders/${serializePathParameter(reminderId, { name: 'reminderId', style: 'simple', explode: false })}/snooze`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'command' });
  }
}

export interface MissoryStoriesListParams {
  page?: number;
  pageSize?: number;
  q?: string;
}

export class MissoryStoriesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a story bundling participants, events, and memories. */
  async create(body: MissoryStoryUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryStory> {
    return this.client.request<MissoryStory>(appApiPath(`/app/v3/api/missory/stories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** List stories. */
  async list(params?: MissoryStoriesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: MissoryStory[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MissoryStory[]; pageInfo: PageInfo; }>(appendQueryString(appApiPath(`/app/v3/api/missory/stories`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Retrieve one story with its timeline. */
  async retrieve(storyId: Int64Id, requestOptions?: ApiRequestOptions): Promise<MissoryStory> {
    return this.client.request<MissoryStory>(appApiPath(`/app/v3/api/missory/stories/${serializePathParameter(storyId, { name: 'storyId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update one story. */
  async update(storyId: Int64Id, body: MissoryStoryUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryStory> {
    return this.client.request<MissoryStory>(appApiPath(`/app/v3/api/missory/stories/${serializePathParameter(storyId, { name: 'storyId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete one story. */
  async delete(storyId: Int64Id, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(appApiPath(`/app/v3/api/missory/stories/${serializePathParameter(storyId, { name: 'storyId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export interface MissoryMemoriesListParams {
  page?: number;
  pageSize?: number;
  q?: string;
  personId?: Int64Id;
  type_?: MissoryMemoryType;
  status?: MissoryMemoryStatus;
  origin?: MissoryMemoryOrigin;
}

export class MissoryMemoriesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Record a memory manually. */
  async create(body: MissoryMemoryUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryMemory> {
    return this.client.request<MissoryMemory>(appApiPath(`/app/v3/api/missory/memories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** List memories with person/type/status/origin/keyword filters. */
  async list(params?: MissoryMemoriesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: MissoryMemory[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'personId', value: params?.personId, style: 'form', explode: true, allowReserved: false },
      { name: 'type', value: params?.type_, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'origin', value: params?.origin, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MissoryMemory[]; pageInfo: PageInfo; }>(appendQueryString(appApiPath(`/app/v3/api/missory/memories`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Extract candidate memories from free text (AI assist, candidates only). */
  async extract(body: MissoryMemoryExtractRequest, requestOptions?: ApiRequestOptions): Promise<{ items: MissoryMemory[]; pageInfo: PageInfo; }> {
    return this.client.request<{ items: MissoryMemory[]; pageInfo: PageInfo; }>(appApiPath(`/app/v3/api/missory/memories/extract`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'page' });
  }

/** Retrieve one memory including source and inference metadata. */
  async retrieve(memoryId: Int64Id, requestOptions?: ApiRequestOptions): Promise<MissoryMemory> {
    return this.client.request<MissoryMemory>(appApiPath(`/app/v3/api/missory/memories/${serializePathParameter(memoryId, { name: 'memoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update one memory. */
  async update(memoryId: Int64Id, body: MissoryMemoryUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryMemory> {
    return this.client.request<MissoryMemory>(appApiPath(`/app/v3/api/missory/memories/${serializePathParameter(memoryId, { name: 'memoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete one memory. */
  async delete(memoryId: Int64Id, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(appApiPath(`/app/v3/api/missory/memories/${serializePathParameter(memoryId, { name: 'memoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }

/** Confirm a candidate or inference memory into a user-confirmed fact. */
  async confirm(memoryId: Int64Id, requestOptions?: ApiRequestOptions): Promise<SdkWorkCommandData> {
    return this.client.request<SdkWorkCommandData>(appApiPath(`/app/v3/api/missory/memories/${serializePathParameter(memoryId, { name: 'memoryId', style: 'simple', explode: false })}/confirm`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'command' });
  }

/** Reject a candidate or inference memory. */
  async reject(memoryId: Int64Id, requestOptions?: ApiRequestOptions): Promise<SdkWorkCommandData> {
    return this.client.request<SdkWorkCommandData>(appApiPath(`/app/v3/api/missory/memories/${serializePathParameter(memoryId, { name: 'memoryId', style: 'simple', explode: false })}/reject`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'command' });
  }
}

export class MissoryRelationshipsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Add or update the owner-to-person relationship. */
  async create(personId: Int64Id, body: MissoryRelationshipUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryRelationship> {
    return this.client.request<MissoryRelationship>(appApiPath(`/app/v3/api/missory/persons/${serializePathParameter(personId, { name: 'personId', style: 'simple', explode: false })}/relationships`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Remove one relationship edge. */
  async delete(personId: Int64Id, relationshipId: Int64Id, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(appApiPath(`/app/v3/api/missory/persons/${serializePathParameter(personId, { name: 'personId', style: 'simple', explode: false })}/relationships/${serializePathParameter(relationshipId, { name: 'relationshipId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class MissoryPersonsTimelineApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Relationship timeline for one person. */
  async list(personId: Int64Id, requestOptions?: ApiRequestOptions): Promise<{ items: MissoryTimelineEntry[]; pageInfo: PageInfo; }> {
    return this.client.request<{ items: MissoryTimelineEntry[]; pageInfo: PageInfo; }>(appApiPath(`/app/v3/api/missory/persons/${serializePathParameter(personId, { name: 'personId', style: 'simple', explode: false })}/timeline`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface MissoryPersonsListParams {
  page?: number;
  pageSize?: number;
  q?: string;
  relationshipType?: MissoryRelationshipType;
}

export class MissoryPersonsApi {
  private client: HttpClient;
  public readonly timeline: MissoryPersonsTimelineApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.timeline = new MissoryPersonsTimelineApi(client);
  }


/** Create a person. */
  async create(body: MissoryPersonUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryPerson> {
    return this.client.request<MissoryPerson>(appApiPath(`/app/v3/api/missory/persons`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** List persons with optional keyword and relationship-type filters. */
  async list(params?: MissoryPersonsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: MissoryPerson[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'relationshipType', value: params?.relationshipType, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MissoryPerson[]; pageInfo: PageInfo; }>(appendQueryString(appApiPath(`/app/v3/api/missory/persons`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Retrieve a person with relationships, recent memories, and stats. */
  async retrieve(personId: Int64Id, requestOptions?: ApiRequestOptions): Promise<MissoryPersonDetail> {
    return this.client.request<MissoryPersonDetail>(appApiPath(`/app/v3/api/missory/persons/${serializePathParameter(personId, { name: 'personId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update a person. */
  async update(personId: Int64Id, body: MissoryPersonUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryPerson> {
    return this.client.request<MissoryPerson>(appApiPath(`/app/v3/api/missory/persons/${serializePathParameter(personId, { name: 'personId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete a person and its owned relationships and memory links. */
  async delete(personId: Int64Id, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(appApiPath(`/app/v3/api/missory/persons/${serializePathParameter(personId, { name: 'personId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class MissoryMyProfileApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve the owner profile of the current user. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<MissoryMyProfile> {
    return this.client.request<MissoryMyProfile>(appApiPath(`/app/v3/api/missory/my_profile`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update the owner profile of the current user. */
  async update(body: MissoryMyProfileUpsertRequest, requestOptions?: ApiRequestOptions): Promise<MissoryMyProfile> {
    return this.client.request<MissoryMyProfile>(appApiPath(`/app/v3/api/missory/my_profile`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryApi {
  public readonly myProfile: MissoryMyProfileApi;
  public readonly persons: MissoryPersonsApi;
  public readonly relationships: MissoryRelationshipsApi;
  public readonly memories: MissoryMemoriesApi;
  public readonly stories: MissoryStoriesApi;
  public readonly reminders: MissoryRemindersApi;
  public readonly home: MissoryHomeApi;

  constructor(client: HttpClient) {
    this.myProfile = new MissoryMyProfileApi(client);
    this.persons = new MissoryPersonsApi(client);
    this.relationships = new MissoryRelationshipsApi(client);
    this.memories = new MissoryMemoriesApi(client);
    this.stories = new MissoryStoriesApi(client);
    this.reminders = new MissoryRemindersApi(client);
    this.home = new MissoryHomeApi(client);
  }

}

export function createMissoryApi(client: HttpClient): MissoryApi {
  return new MissoryApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
