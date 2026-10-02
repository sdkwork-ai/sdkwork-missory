import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

test("app.json declares the five P0 pages", () => {
  const app = JSON.parse(fs.readFileSync(path.join(root, "src/app.json"), "utf8"));
  for (const page of ["pages/home/index", "pages/people/index", "pages/person/index", "pages/memories/index", "pages/assistant/index"]) {
    assert.ok(app.pages.includes(page), `missing page: ${page}`);
  }
});

test("runtime env docs exist for all ten profiles with the four SDKWORK keys", () => {
  for (const profile of ["standalone", "cloud"]) {
    for (const env of ["development", "test", "staging", "demo", "production"]) {
      const file = path.join(root, "config/mini-program", `runtime-env.${profile}.${env}.json`);
      const doc = JSON.parse(fs.readFileSync(file, "utf8"));
      for (const key of ["SDKWORK_ENVIRONMENT", "SDKWORK_DEPLOYMENT_PROFILE", "SDKWORK_PROFILE_ID", "SDKWORK_RUNTIME_TARGET"]) {
        assert.ok(doc[key], `${file} misses ${key}`);
      }
      assert.equal(doc.SDKWORK_RUNTIME_TARGET, "mini-program");
    }
  }
});

test("pages never call wx.request directly (host adapter only)", () => {
  const pagesDir = path.join(root, "src/pages");
  for (const entry of fs.readdirSync(pagesDir)) {
    const js = fs.readFileSync(path.join(pagesDir, entry, "index.js"), "utf8");
    assert.ok(!js.includes("wx.request"), `pages/${entry} must use the runtime services, not wx.request`);
  }
});

test("mp-core builds the generated SDK client over the host fetch adapter", () => {
  const runtime = fs.readFileSync(path.join(root, "packages/sdkwork-missory-mp-core/src/runtime.ts"), "utf8");
  assert.match(runtime, /createClient\(/u);
  assert.match(runtime, /createWxFetchAdapter()/u);
});
