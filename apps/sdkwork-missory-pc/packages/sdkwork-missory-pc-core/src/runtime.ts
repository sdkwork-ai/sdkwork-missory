import { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import type { MissoryPcRuntimeConfig } from "./config/runtime-config.ts";
import { createTokenManagerFor } from "./session/tokenManager.ts";
import { createMissorySessionFacade, type MissorySessionFacade } from "./session/iamAuth.ts";
import { createPeopleService } from "./services/people-service.ts";
import { createMemoriesService } from "./services/memories-service.ts";
import { createAssistantService } from "./services/assistant-service.ts";
import { createHomeService } from "./services/home-service.ts";
import { createStoriesService } from "./services/stories-service.ts";
import { createProfileService } from "./services/profile-service.ts";

export interface MissoryPcRuntime {
  config: MissoryPcRuntimeConfig;
  client: SdkworkAppClient;
  session: MissorySessionFacade;
  people: ReturnType<typeof createPeopleService>;
  memories: ReturnType<typeof createMemoriesService>;
  assistant: ReturnType<typeof createAssistantService>;
  home: ReturnType<typeof createHomeService>;
  stories: ReturnType<typeof createStoriesService>;
  profile: ReturnType<typeof createProfileService>;
}

export function createMissoryPcRuntime(config: MissoryPcRuntimeConfig): MissoryPcRuntime {
  // IAM dual-token runtime: the token manager carries the stored login pair
  // (or the development bypass identity) and every request dispatches
  // Authorization/Access-Token through it (TECH_ARCHITECTURE section 8).
  const tokenManager = createTokenManagerFor(config);
  const client = createClient({
    baseUrl: config.appApiBaseUrl,
    platform: "pc",
    authMode: "dual-token",
    tokenManager,
  });
  return {
    config,
    client,
    session: createMissorySessionFacade(config, tokenManager),
    people: createPeopleService(client),
    memories: createMemoriesService(client),
    assistant: createAssistantService(client),
    home: createHomeService(client),
    stories: createStoriesService(client),
    profile: createProfileService(client),
  };
}
