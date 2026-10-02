#!/usr/bin/env python3
"""One-shot scaffolder for the missory PC workspace packages (run from repo root)."""
import json
import os

BASE = "apps/sdkwork-missory-pc/packages"

def w(path, content):
    full = os.path.join(BASE, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8", newline="\n") as f:
        f.write(content)

def pkg_json(name, deps):
    return json.dumps({
        "name": name, "version": "0.1.0", "private": True, "type": "module",
        "main": "./src/index.ts", "module": "./src/index.ts", "types": "./src/index.ts",
        "exports": {".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"}},
        "dependencies": deps,
    }, indent=2, ensure_ascii=False) + "\n"

# ---------------- pc-core ----------------
w("sdkwork-missory-pc-core/package.json", pkg_json("@sdkwork/missory-pc-core", {
    "@sdkwork/missory-app-sdk": "workspace:*",
}))

w("sdkwork-missory-pc-core/src/index.ts", '''export { loadMissoryPcRuntimeConfig, parseMissoryPcRuntimeConfig } from "./config/runtime-config.ts";
export type { MissoryPcRuntimeConfig } from "./config/runtime-config.ts";
export { createMissoryPcRuntime } from "./runtime.ts";
export type { MissoryPcRuntime } from "./runtime.ts";
export { normalizeClientError, MissoryClientError } from "./errors.ts";
export * from "./services/index.ts";
''')

w("sdkwork-missory-pc-core/src/config/runtime-config.ts", '''export type MissoryEnvironment = "development" | "test" | "staging" | "demo" | "production";
export type MissoryDeploymentProfile = "standalone" | "cloud";

export interface MissoryPcRuntimeConfig {
  environment: MissoryEnvironment;
  deploymentProfile: MissoryDeploymentProfile;
  profileId: string;
  runtimeTarget: "browser";
  browserOriginMode: "same-origin" | "cross-origin";
  defaultLocale: string;
  fallbackLocale: string;
  supportedLocales: string[];
  appApiBaseUrl: string;
  backendApiBaseUrl: string;
  openApiBaseUrl: string;
}

const ENVIRONMENTS: MissoryEnvironment[] = ["development", "test", "staging", "demo", "production"];
const PROFILES: MissoryDeploymentProfile[] = ["standalone", "cloud"];

function requireText(value: unknown, field: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`runtime-env field ${field} is missing`);
  }
  return value;
}

function requireHttpUrl(value: unknown, field: string): string {
  const text = requireText(value, field);
  const parsed = new URL(text);
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error(`runtime-env field ${field} must be an http(s) URL`);
  }
  if (
    (parsed.hostname === "127.0.0.1" || parsed.hostname === "localhost") &&
    process.env.NODE_ENV === "production"
  ) {
    throw new Error(`runtime-env field ${field} must not point at loopback in production`);
  }
  return text.replace(/\\/+$/, "");
}

export function parseMissoryPcRuntimeConfig(input: unknown): MissoryPcRuntimeConfig {
  if (!input || typeof input !== "object") {
    throw new Error("runtime-env document is not an object");
  }
  const doc = input as Record<string, unknown>;
  const environment = requireText(doc.environment, "environment") as MissoryEnvironment;
  const deploymentProfile = requireText(doc.deploymentProfile, "deploymentProfile") as MissoryDeploymentProfile;
  if (!ENVIRONMENTS.includes(environment)) {
    throw new Error(`runtime-env environment is invalid: ${environment}`);
  }
  if (!PROFILES.includes(deploymentProfile)) {
    throw new Error(`runtime-env deploymentProfile is invalid: ${deploymentProfile}`);
  }
  const originMode = requireText(doc.browserOriginMode, "browserOriginMode");
  if (originMode !== "same-origin" && originMode !== "cross-origin") {
    throw new Error(`runtime-env browserOriginMode is invalid: ${originMode}`);
  }
  return {
    environment,
    deploymentProfile,
    profileId: requireText(doc.profileId, "profileId"),
    runtimeTarget: "browser",
    browserOriginMode: originMode,
    defaultLocale: requireText(doc.defaultLocale, "defaultLocale"),
    fallbackLocale: requireText(doc.fallbackLocale, "fallbackLocale"),
    supportedLocales: Array.isArray(doc.supportedLocales) ? (doc.supportedLocales as string[]) : ["zh-CN"],
    appApiBaseUrl: requireHttpUrl(doc.appApiBaseUrl, "appApiBaseUrl"),
    backendApiBaseUrl: requireHttpUrl(doc.backendApiBaseUrl, "backendApiBaseUrl"),
    openApiBaseUrl: requireHttpUrl(doc.openApiBaseUrl, "openApiBaseUrl"),
  };
}

export async function loadMissoryPcRuntimeConfig(
  fetcher: typeof fetch = fetch,
): Promise<MissoryPcRuntimeConfig> {
  const response = await fetcher("/runtime-env.json", { cache: "no-store", credentials: "same-origin" });
  if (!response.ok) {
    throw new Error(`Runtime configuration failed with HTTP ${response.status}`);
  }
  return parseMissoryPcRuntimeConfig(await response.json());
}
''')

w("sdkwork-missory-pc-core/src/runtime.ts", '''import { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import type { MissoryPcRuntimeConfig } from "./config/runtime-config.ts";
import { createPeopleService } from "./services/people-service.ts";
import { createMemoriesService } from "./services/memories-service.ts";
import { createAssistantService } from "./services/assistant-service.ts";
import { createHomeService } from "./services/home-service.ts";
import { createStoriesService } from "./services/stories-service.ts";
import { createProfileService } from "./services/profile-service.ts";

export interface MissoryPcRuntime {
  config: MissoryPcRuntimeConfig;
  client: SdkworkAppClient;
  people: ReturnType<typeof createPeopleService>;
  memories: ReturnType<typeof createMemoriesService>;
  assistant: ReturnType<typeof createAssistantService>;
  home: ReturnType<typeof createHomeService>;
  stories: ReturnType<typeof createStoriesService>;
  profile: ReturnType<typeof createProfileService>;
}

export function createMissoryPcRuntime(config: MissoryPcRuntimeConfig): MissoryPcRuntime {
  // Phase-1 adoption note: the IAM dual-token adapter is not wired yet; the
  // standalone development gateway injects the owner context (see TECH
  // ARCHITECTURE section 8). The client accepts a tokenManager again when the
  // IAM runtime lands.
  const client = createClient({ baseUrl: config.appApiBaseUrl, platform: "pc" });
  return {
    config,
    client,
    people: createPeopleService(client),
    memories: createMemoriesService(client),
    assistant: createAssistantService(client),
    home: createHomeService(client),
    stories: createStoriesService(client),
    profile: createProfileService(client),
  };
}
''')

w("sdkwork-missory-pc-core/src/errors.ts", '''/** Normalized client failure carrying the problem-detail code when present. */
export class MissoryClientError extends Error {
  readonly status: number;
  readonly code: number | null;
  readonly traceId: string | null;

  constructor(message: string, status: number, code: number | null, traceId: string | null) {
    super(message);
    this.name = "MissoryClientError";
    this.status = status;
    this.code = code;
    this.traceId = traceId;
  }
}

export function normalizeClientError(error: unknown): MissoryClientError {
  if (error instanceof MissoryClientError) return error;
  const payload = error as { status?: unknown; code?: unknown; message?: unknown; traceId?: unknown; body?: unknown };
  const status = typeof payload?.status === "number" ? payload.status : 0;
  const code = typeof payload?.code === "number" ? payload.code : null;
  const traceId = typeof payload?.traceId === "string" ? payload.traceId : null;
  const detail =
    (payload?.body as { detail?: unknown })?.detail ??
    (typeof payload?.message === "string" ? payload.message : undefined);
  return new MissoryClientError(
    typeof detail === "string" ? detail : "请求失败，请稍后重试",
    status,
    code,
    traceId,
  );
}
''')

w("sdkwork-missory-pc-core/src/services/index.ts", '''export { createPeopleService } from "./people-service.ts";
export { createMemoriesService } from "./memories-service.ts";
export { createAssistantService } from "./assistant-service.ts";
export { createHomeService } from "./home-service.ts";
export { createStoriesService } from "./stories-service.ts";
export { createProfileService } from "./profile-service.ts";
''')

w("sdkwork-missory-pc-core/src/services/people-service.ts", '''import type { MissoryPerson, MissoryPersonDetail, MissoryPersonUpsertRequest, MissoryRelationship, MissoryRelationshipUpsertRequest, MissoryTimelineEntry, PageInfo, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

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
    timeline(personId: string) {
      return client.missory.persons.timeline
        .list(personId)
        .then((page) => page.items as MissoryTimelineEntry[]);
    },
    upsertRelationship(personId: string, request: MissoryRelationshipUpsertRequest): Promise<MissoryRelationship> {
      return client.missory.relationships.create(personId, request);
    },
    deleteRelationship(personId: string, relationshipId: string): Promise<void> {
      return client.missory.relationships.delete(personId, relationshipId);
    },
  };
}
''')

w("sdkwork-missory-pc-core/src/services/memories-service.ts", '''import type { MissoryMemory, MissoryMemoryExtractRequest, MissoryMemoryUpsertRequest, PageInfo, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

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
        type: query.type as never,
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
''')

w("sdkwork-missory-pc-core/src/services/assistant-service.ts", '''import type { SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createAssistantService(client: SdkworkAppClient) {
  return {
    query(question: string) {
      return client.missoryAssistant.assistant.query({ question });
    },
    briefing(personId: string) {
      return client.missoryAssistant.briefings.create({ personId });
    },
    messageDraft(personId: string, scenario: string, tone?: string, note?: string) {
      return client.missoryAssistant.messageDrafts.create({
        personId,
        scenario: scenario as never,
        ...(tone ? { tone: tone as never } : {}),
        ...(note ? { note } : {}),
      });
    },
    chatSummary(personId: string, text: string) {
      return client.missoryAssistant.chatSummaries.create({ personId, text });
    },
    storySummary(storyId: string) {
      return client.missoryAssistant.stories.summaries.create(storyId);
    },
  };
}
''')

w("sdkwork-missory-pc-core/src/services/home-service.ts", '''import type { SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createHomeService(client: SdkworkAppClient) {
  return {
    today() {
      return client.missory.home.today.list();
    },
    reminders(params: { page?: number; pageSize?: number; type?: string } = {}) {
      return client.missory.reminders.list({
        page: params.page,
        pageSize: params.pageSize,
        type: params.type as never,
      });
    },
    dismissReminder(reminderId: string) {
      return client.missory.reminders.dismiss(reminderId);
    },
    snoozeReminder(reminderId: string, days: number) {
      return client.missory.reminders.snooze(reminderId, { days });
    },
  };
}
''')

w("sdkwork-missory-pc-core/src/services/stories-service.ts", '''import type { MissoryStoryUpsertRequest, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

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
''')

w("sdkwork-missory-pc-core/src/services/profile-service.ts", '''import type { MissoryMyProfileUpsertRequest, SdkworkAppClient } from "@sdkwork/missory-app-sdk";

export function createProfileService(client: SdkworkAppClient) {
  return {
    retrieve() {
      return client.missory.myProfile.retrieve();
    },
    update(request: MissoryMyProfileUpsertRequest) {
      return client.missory.myProfile.update(request);
    },
  };
}
''')

w("sdkwork-missory-pc-core/README.md", '''# @sdkwork/missory-pc-core

PC core package: runtime-config loading, the generated app-sdk client factory,
and typed services. The only package allowed to construct SDK clients
(APP_SDK_INTEGRATION_SPEC section 9); screens and the shell receive services
through the bootstrapped runtime.

Canonical specs: `../../../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`.
''')

print("core package written")
