// Live-gateway integration test for the mini-program session facade.
//
//   SDKWORK_E2E_LIVE=1 SDKWORK_E2E_BASE_URL=http://127.0.0.1:8464 node --test tests/live-gateway.test.mjs
//
// Exercises the REAL mp-core session facade (credential-entry password login
// through the gateway's injected runtime-env bootstrap credential, dual-token
// persistence in the wx storage shim) plus a dual-token business call.
// Without SDKWORK_E2E_LIVE the test skips, keeping offline CI green.
import { test } from "node:test";
import assert from "node:assert/strict";

const BASE_URL = process.env.SDKWORK_E2E_BASE_URL ?? "http://127.0.0.1:8464";

test("live gateway: mp session facade login + dual-token business call", { skip: process.env.SDKWORK_E2E_LIVE !== "1" }, async () => {
  // wx storage shim (the module reads wx.* lazily inside functions).
  const storage = new Map();
  globalThis.wx = {
    getStorageSync: (key) => storage.get(key) ?? "",
    setStorageSync: (key, value) => storage.set(key, value),
    removeStorageSync: (key) => storage.delete(key),
  };

  const environment = await (await fetch(`${BASE_URL}/runtime-env.json`)).json();
  const bootstrap = environment.authBootstrapAccessToken;
  assert.equal(typeof bootstrap, "string", "gateway must inject the bootstrap credential");
  assert.equal(bootstrap.split(".").length, 3, "bootstrap must be a JWT");

  // Register a fresh account through the credential-entry surface.
  const username = `mp-e2e-${Date.now()}@missory.test`;
  const registration = await fetch(`${BASE_URL}/app/v3/api/auth/registrations`, {
    method: "POST",
    headers: { "content-type": "application/json", "Access-Token": bootstrap },
    body: JSON.stringify({
      username,
      password: "Mp-Passw0rd!",
      confirmPassword: "Mp-Passw0rd!",
      displayName: "MP E2E",
    }),
  }).then((response) => response.json());
  assert.equal(registration.code, 0, "registration must succeed");
  assert.match(String(registration.data.user.id), /^\d+$/, "int64 ids serialize as strings");

  // Import by module path: the package barrel re-exports a type-only SDK
  // binding that node's type stripping cannot execute (esbuild handles it in
  // the real runtime bundle).
  const { createMissoryMpSessionFacade, createMissoryMpTokenManager } = await import(
    "../packages/sdkwork-missory-mp-core/src/session.ts"
  );
  const mpEnvironment = {
    environment: "development",
    deploymentProfile: "standalone",
    profileId: "standalone.development",
    appApiBaseUrl: BASE_URL,
    authBootstrapAccessToken: bootstrap,
  };
  const facade = createMissoryMpSessionFacade(mpEnvironment, createMissoryMpTokenManager(mpEnvironment));
  const session = await facade.loginWithPassword({ account: username, password: "Mp-Passw0rd!" });
  assert.ok(session.authToken.length > 0);
  assert.ok(facade.isAuthenticated());

  // Dual-token business call resolves the real principal.
  const people = await fetch(`${BASE_URL}/app/v3/api/missory/persons?page=1&page_size=20`, {
    headers: { Authorization: `Bearer ${session.authToken}`, "Access-Token": session.accessToken },
  }).then((response) => response.json());
  assert.equal(people.code, 0);
  assert.ok(Array.isArray(people.data.items));
  assert.ok(people.data.pageInfo, "list responses carry PageInfo");

  facade.logout();
  assert.equal(facade.isAuthenticated(), false);
  assert.equal(storage.size, 0, "logout clears the wx storage session");
});
