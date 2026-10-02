// Session-expiry boundary: watches app-api responses for 401/403 and
// dispatches SESSION_EXPIRED_EVENT so the UI session gate can clear the
// stored session and return to login. Implemented as a one-time window.fetch
// wrapper because the generated SDK client is generator-owned output.

import { SESSION_EXPIRED_EVENT } from "./iamAuth.ts";

const APP_API_MARKER = "/app/v3/api/";

let installed = false;

export function installSessionExpiryBoundary(fetcher: typeof fetch = fetch): () => void {
  if (installed) return () => undefined;
  installed = true;
  const original = fetcher.bind(globalThis);
  const wrapped: typeof fetch = async (input, init) => {
    const response = await original(input, init);
    const url = typeof input === "string"
      ? input
      : input instanceof URL
        ? input.href
        : (input.url ?? "");
    if ((response.status === 401 || response.status === 403) && url.includes(APP_API_MARKER)) {
      globalThis.dispatchEvent(new CustomEvent(SESSION_EXPIRED_EVENT));
    }
    return response;
  };
  globalThis.fetch = wrapped;
  return () => {
    globalThis.fetch = original;
    installed = false;
  };
}
