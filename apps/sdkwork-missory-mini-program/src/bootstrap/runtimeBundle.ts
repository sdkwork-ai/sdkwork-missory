// The single safe runtime module bundled by esbuild into src/runtime/runtime.js
// (MINI_PROGRAM_APP_ARCHITECTURE_SPEC section 5): host adapter registration,
// SDK client construction, and services exposed to page TS through one object.
import { bootstrapMissoryMpRuntime } from "@sdkwork/missory-mp-core";

// Error primitives live in mp-commons (package taxonomy: domain-neutral error
// primitives); pages reach them through the runtime module because page
// sources must not require package internals directly.
export { resolveErrorMessage } from "@sdkwork/missory-mp-commons";

const env = {
  environment: "__SDKWORK_MISSORY_ENVIRONMENT__",
  deploymentProfile: "__SDKWORK_MISSORY_DEPLOYMENT_PROFILE__",
  profileId: "__SDKWORK_MISSORY_PROFILE_ID__",
  appApiBaseUrl: "__SDKWORK_MISSORY_MP_APP_API_BASE_URL__",
  authBootstrapAccessToken: "__SDKWORK_MISSORY_AUTH_BOOTSTRAP_ACCESS_TOKEN__",
};

const runtime = bootstrapMissoryMpRuntime(env);

// Pages destructure `{ page }` from the app-level runtime module; keep the
// namespace alias so page JS stays `page.client.missory...`.
export const page = runtime;

export function bootstrap() {
  return runtime;
}

export default runtime;
