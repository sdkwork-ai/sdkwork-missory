import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (p) => fs.readFileSync(path.join(root, p), "utf8");

test("only pc-core constructs SDK clients", () => {
  const packagesDir = path.join(root, "packages");
  for (const entry of fs.readdirSync(packagesDir)) {
    if (entry === "sdkwork-missory-h5-core") continue;
    for (const file of walk(path.join(packagesDir, entry, "src"))) {
      const text = fs.readFileSync(file, "utf8");
      // DTO `import type` statements are fine; constructing clients or
      // value-importing the SDK facade is reserved for pc-core
      // (APP_SDK_INTEGRATION_SPEC section 9).
      assert.ok(!/createClient\(/u.test(text), `${file} must not construct SDK clients`);
      const withoutTypeImports = text.replace(/import\s+type\s+[\s\S]*?from\s+"[^"]*"\s*;?/gu, "");
      assert.ok(
        !withoutTypeImports.includes("@sdkwork/missory-app-sdk"),
        `${file} must value-import the SDK only via pc-core`,
      );
    }
  }
});

test("vite config uses canonical dist layout helper", () => {
  const vite = read("vite.config.ts");
  assert.match(vite, /resolveBrowserDistOutDir/u);
  assert.match(vite, /resolveViteEnvironment/u);
  assert.ok(!/resolve:\s*\{[^}]*alias/u.test(vite), "no resolve.alias for packages");
});

test("runtime-env source docs exist for all ten profiles", () => {
  for (const profile of ["standalone", "cloud"]) {
    for (const env of ["development", "test", "staging", "demo", "production"]) {
      const file = path.join(root, "etc/browser", `runtime-env.${profile}.${env}.json`);
      const doc = JSON.parse(fs.readFileSync(file, "utf8"));
      assert.equal(doc.profileId, `${profile}.${env}`);
      assert.equal(doc.runtimeTarget, "browser");
    }
  }
});

function walk(dir) {
  if (!fs.existsSync(dir)) return [];
  const out = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) out.push(...walk(full));
    else if (entry.name.endsWith(".ts") || entry.name.endsWith(".tsx")) out.push(full);
  }
  return out;
}

test("session composition: gate uses shell LoginScreen and core session facade", () => {
  const app = read("src/App.tsx");
  assert.match(app, /SessionGate/u, "App must gate screens behind the session");
  assert.match(app, /LoginScreen/u, "gate renders the shell LoginScreen");
  assert.match(app, /SESSION_EXPIRED_EVENT/u, "gate listens for the session-expiry event");
  const shellIndex = read("packages/sdkwork-missory-h5-shell/src/index.ts");
  assert.match(shellIndex, /LoginScreen/u, "shell exports LoginScreen");
  const coreIndex = read("packages/sdkwork-missory-h5-core/src/index.ts");
  assert.match(coreIndex, /createMissorySessionFacade/u, "core exports the session facade");
  const tokenManager = read("packages/sdkwork-missory-h5-core/src/session/tokenManager.ts");
  assert.match(tokenManager, /loadStoredSession/u, "boot restores the persisted session");
});
