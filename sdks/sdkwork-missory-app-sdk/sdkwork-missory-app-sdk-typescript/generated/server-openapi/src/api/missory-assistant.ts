import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { Int64Id, MissoryAssistantAnswer, MissoryAssistantQueryRequest, MissoryBriefing, MissoryBriefingRequest, MissoryChatSummary, MissoryChatSummaryRequest, MissoryMessageDraft, MissoryMessageDraftRequest, MissoryStory } from '../types';


export class MissoryAssistantAssistantChatSummariesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Summarize pasted chat text and produce candidate memories. */
  async create(body: MissoryChatSummaryRequest, requestOptions?: ApiRequestOptions): Promise<MissoryChatSummary> {
    return this.client.request<MissoryChatSummary>(appApiPath(`/app/v3/api/missory/assistant/chat_summaries`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryAssistantAssistantMessageDraftsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Draft a message (birthday, check-in) — draft only, never auto-sent. */
  async create(body: MissoryMessageDraftRequest, requestOptions?: ApiRequestOptions): Promise<MissoryMessageDraft> {
    return this.client.request<MissoryMessageDraft>(appApiPath(`/app/v3/api/missory/assistant/message_drafts`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryAssistantAssistantBriefingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Build a meeting briefing for one person. */
  async create(body: MissoryBriefingRequest, requestOptions?: ApiRequestOptions): Promise<MissoryBriefing> {
    return this.client.request<MissoryBriefing>(appApiPath(`/app/v3/api/missory/assistant/briefings`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryAssistantAssistantApi {
  private client: HttpClient;
  public readonly briefings: MissoryAssistantAssistantBriefingsApi;
  public readonly messageDrafts: MissoryAssistantAssistantMessageDraftsApi;
  public readonly chatSummaries: MissoryAssistantAssistantChatSummariesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.briefings = new MissoryAssistantAssistantBriefingsApi(client);
    this.messageDrafts = new MissoryAssistantAssistantMessageDraftsApi(client);
    this.chatSummaries = new MissoryAssistantAssistantChatSummariesApi(client);
  }


/** Ask the social agent a person/relationship/memory question. */
  async query(body: MissoryAssistantQueryRequest, requestOptions?: ApiRequestOptions): Promise<MissoryAssistantAnswer> {
    return this.client.request<MissoryAssistantAnswer>(appApiPath(`/app/v3/api/missory/assistant/query`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryAssistantStoriesSummariesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create (or regenerate) the AI summary of a story. */
  async create(storyId: Int64Id, requestOptions?: ApiRequestOptions): Promise<MissoryStory> {
    return this.client.request<MissoryStory>(appApiPath(`/app/v3/api/missory/stories/${serializePathParameter(storyId, { name: 'storyId', style: 'simple', explode: false })}/summaries`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MissoryAssistantStoriesApi {
  public readonly summaries: MissoryAssistantStoriesSummariesApi;

  constructor(client: HttpClient) {
    this.summaries = new MissoryAssistantStoriesSummariesApi(client);
  }

}

export class MissoryAssistantApi {
  public readonly stories: MissoryAssistantStoriesApi;
  public readonly assistant: MissoryAssistantAssistantApi;

  constructor(client: HttpClient) {
    this.stories = new MissoryAssistantStoriesApi(client);
    this.assistant = new MissoryAssistantAssistantApi(client);
  }

}

export function createMissoryAssistantApi(client: HttpClient): MissoryAssistantApi {
  return new MissoryAssistantApi(client);
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
