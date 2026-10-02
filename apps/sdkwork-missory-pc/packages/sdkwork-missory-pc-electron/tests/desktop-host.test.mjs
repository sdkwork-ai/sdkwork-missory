import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

test("desktop host is a shell only: no business imports in main.mjs", () => {
  const main = fs.readFileSync(path.join(root, "src/main.mjs"), "utf8");
  for (const forbidden of ["@sdkwork/missory-app-sdk", "fetch(", "axios", "http.request"]) {
    assert.ok(!main.includes(forbidden), `desktop host must not contain ${forbidden}`);
  }
  assert.match(main, /contextIsolation:\s*true/u);
  assert.match(main, /nodeIntegration:\s*false/u);
  assert.match(main, /sandbox:\s*true/u);
});

test("package.json names the electron host family", () => {
  const manifest = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8"));
  assert.equal(manifest.name, "@sdkwork/missory-pc-electron");
});
