import { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

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
