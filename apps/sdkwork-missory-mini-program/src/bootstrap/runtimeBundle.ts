// The single safe runtime module bundled by esbuild into src/runtime/runtime.js
// (MINI_PROGRAM_APP_ARCHITECTURE_SPEC section 5): host adapter registration,
// SDK client construction, and services exposed to page JS through one object.
import { bootstrapMissoryMpRuntime } from "@sdkwork/missory-mp-core";

const env = {
  environment: "__SDKWORK_MISSORY_ENVIRONMENT__",
  deploymentProfile: "__SDKWORK_MISSORY_DEPLOYMENT_PROFILE__",
  profileId: "__SDKWORK_MISSORY_PROFILE_ID__",
  appApiBaseUrl: "__SDKWORK_MISSORY_MP_APP_API_BASE_URL__",
};

const runtime = bootstrapMissoryMpRuntime(env);

export function bootstrap() {
  return runtime;
}

export default runtime;
