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
    const ts = fs.readFileSync(path.join(pagesDir, entry, "index.ts"), "utf8");
    assert.ok(!ts.includes("wx.request"), `pages/${entry} must use the runtime services, not wx.request`);
  }
});

test("authored sources are TypeScript; platform JavaScript is generated output only", () => {
  // MINI_PROGRAM_APP_ARCHITECTURE_SPEC 1.1 language boundary: hand-authored
  // page, app, and package sources are .ts; the only JavaScript under src/ is
  // the esbuild runtime bundle owned by scripts/build-runtime.mjs.
  assert.ok(fs.existsSync(path.join(root, "src/app.ts")), "src/app.ts must exist");
  assert.ok(!fs.existsSync(path.join(root, "src/app.js")), "src/app.js must not be hand-authored");
  const pagesDir = path.join(root, "src/pages");
  for (const entry of fs.readdirSync(pagesDir)) {
    assert.ok(fs.existsSync(path.join(pagesDir, entry, "index.ts")), `pages/${entry}/index.ts must exist`);
    assert.ok(!fs.existsSync(path.join(pagesDir, entry, "index.js")), `pages/${entry}/index.js must not be hand-authored`);
  }
  const build = fs.readFileSync(path.join(root, "scripts/build-runtime.mjs"), "utf8");
  assert.match(build, /src\/runtime\/runtime\.js/u, "the runtime bundle outfile must stay build-owned");
  const allowedJs = new Set([path.join(root, "src", "runtime", "runtime.js")]);
  const offenders = [];
  const stack = [path.join(root, "src")];
  while (stack.length > 0) {
    const dir = stack.pop();
    for (const item of fs.readdirSync(dir, { withFileTypes: true })) {
      const resolved = path.join(dir, item.name);
      if (item.isDirectory()) {
        stack.push(resolved);
      } else if (item.name.endsWith(".js") && !allowedJs.has(resolved)) {
        offenders.push(path.relative(root, resolved));
      }
    }
  }
  assert.deepEqual(offenders, [], `hand-authored JavaScript under src/: ${offenders.join(", ")}`);
});

test("tsconfig keeps the strict baseline over the whole authored graph", () => {
  const tsconfig = JSON.parse(fs.readFileSync(path.join(root, "tsconfig.json"), "utf8"));
  assert.equal(tsconfig.compilerOptions.strict, true);
  assert.equal(tsconfig.compilerOptions.noUncheckedIndexedAccess, true);
  assert.equal(tsconfig.compilerOptions.noImplicitReturns, true);
  assert.equal(tsconfig.compilerOptions.noFallthroughCasesInSwitch, true);
  assert.ok(tsconfig.include.includes("src/**/*.ts"), "typecheck must cover the whole authored src/ graph");
});

test("mp-core builds the generated SDK client over the host fetch adapter", () => {
  const runtime = fs.readFileSync(path.join(root, "packages/sdkwork-missory-mp-core/src/runtime.ts"), "utf8");
  assert.match(runtime, /createClient\(/u);
  assert.match(runtime, /createWxFetchAdapter()/u);
});
