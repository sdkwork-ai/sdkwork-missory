// Live-gateway integration test for the PC session facade.
//
//   SDKWORK_E2E_LIVE=1 SDKWORK_E2E_BASE_URL=http://127.0.0.1:8464 node --test tests/live-gateway.test.mjs
//
// Exercises the REAL pc-core session facade (credential-entry password login
// through the gateway's injected runtime-env bootstrap credential, dual-token
// persistence in the localStorage shim) plus a dual-token business call.
// Without SDKWORK_E2E_LIVE the test skips, keeping offline CI green.
import { test } from "node:test";
import assert from "node:assert/strict";

const BASE_URL = process.env.SDKWORK_E2E_BASE_URL ?? "http://127.0.0.1:8464";

test("live gateway: pc session facade login + dual-token business call", { skip: process.env.SDKWORK_E2E_LIVE !== "1" }, async () => {
  // localStorage shim (sessionStore reads globalThis.localStorage lazily).
  const storage = new Map();
  globalThis.localStorage = {
    getItem: (key) => storage.get(key) ?? null,
    setItem: (key, value) => storage.set(key, String(value)),
    removeItem: (key) => storage.delete(key),
    clear: () => storage.clear(),
  };

  const environment = await (await fetch(`${BASE_URL}/runtime-env.json`)).json();
  const bootstrap = environment.authBootstrapAccessToken;
  assert.equal(typeof bootstrap, "string", "gateway must inject the bootstrap credential");
  assert.equal(bootstrap.split(".").length, 3, "bootstrap must be a JWT");

  const username = `pc-e2e-${Date.now()}@missory.test`;
  const registration = await fetch(`${BASE_URL}/app/v3/api/auth/registrations`, {
    method: "POST",
    headers: { "content-type": "application/json", "Access-Token": bootstrap },
    body: JSON.stringify({
      username,
      password: "Pc-Passw0rd!",
      confirmPassword: "Pc-Passw0rd!",
      displayName: "PC E2E",
    }),
  }).then((response) => response.json());
  assert.equal(registration.code, 0, "registration must succeed");
  assert.match(String(registration.data.user.id), /^\d+$/, "int64 ids serialize as strings");

  const { createTokenManager } = await import("@sdkwork/sdk-common");
  const { createMissorySessionFacade } = await import(
    "../packages/sdkwork-missory-pc-core/src/session/iamAuth.ts"
  );
  const config = {
    environment: "development",
    deploymentProfile: "standalone",
    profileId: "standalone.development",
    runtimeTarget: "browser",
    browserOriginMode: "same-origin",
    defaultLocale: "zh-CN",
    fallbackLocale: "en-US",
    supportedLocales: ["zh-CN"],
    appApiBaseUrl: BASE_URL,
    backendApiBaseUrl: BASE_URL,
    openApiBaseUrl: BASE_URL,
    authBootstrapAccessToken: bootstrap,
  };
  const facade = createMissorySessionFacade(config, createTokenManager());
  const session = await facade.loginWithPassword({ account: username, password: "Pc-Passw0rd!" });
  assert.ok(session.authToken.length > 0);
  assert.ok(facade.isAuthenticated());

  const people = await fetch(`${BASE_URL}/app/v3/api/missory/persons?page=1&page_size=20`, {
    headers: { Authorization: `Bearer ${session.authToken}`, "Access-Token": session.accessToken },
  }).then((response) => response.json());
  assert.equal(people.code, 0);
  assert.ok(Array.isArray(people.data.items));
  assert.ok(people.data.pageInfo, "list responses carry PageInfo");

  facade.logout();
  assert.equal(facade.isAuthenticated(), false);
  assert.equal(storage.size, 0, "logout clears the localStorage session");
});
