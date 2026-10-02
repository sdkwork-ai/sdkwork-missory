// Environment resolution for the mini program (MINI_PROGRAM_APP_ARCHITECTURE_SPEC
// bootstrap layout). Values are stamped at build time by scripts/build-runtime.mjs
// from config/mini-program/runtime-env.<profileId>.json.
export interface MissoryMpEnv {
  environment: string;
  deploymentProfile: string;
  profileId: string;
  appApiBaseUrl: string;
}

export function resolveEnvironmentFromGlobalBuildConstants(): MissoryMpEnv {
  return {
    environment: "__SDKWORK_MISSORY_ENVIRONMENT__",
    deploymentProfile: "__SDKWORK_MISSORY_DEPLOYMENT_PROFILE__",
    profileId: "__SDKWORK_MISSORY_PROFILE_ID__",
    appApiBaseUrl: "__SDKWORK_MISSORY_MP_APP_API_BASE_URL__",
  };
}
