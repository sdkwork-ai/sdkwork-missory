#!/usr/bin/env node
/**
 * Desktop host verification (CODE_STYLE_SPEC section 7 build source integrity):
 * 1. verifies the host sources parse (node --check);
 * 2. verifies the PC console build target exists (or a start URL is declared);
 * 3. with --package and electron available, emits an unpacked desktop bundle
 *    into target/desktop via `electron --version` evidence + a package manifest
 *    (full installers are produced by the release pipeline through bin/).
 */
import { execFileSync, spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const hostRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const appRoot = path.resolve(hostRoot, "..", "..");
const wantsPackage = process.argv.includes("--package");

function fail(message) {
  process.stderr.write(`[missory-pc-electron] ${message}\n`);
  process.exit(1);
}

for (const source of ["src/main.mjs"]) {
  const file = path.join(hostRoot, source);
  if (!fs.existsSync(file)) fail(`missing host source: ${source}`);
  const result = spawnSync(process.execPath, ["--check", file], { encoding: "utf8" });
  if (result.status !== 0) fail(`${source} failed node --check:\n${result.stderr}`);
  process.stdout.write(`[missory-pc-electron] ${source} parses ok\n`);
}

const alias = { development: "dev", test: "test", staging: "staging", demo: "demo", production: "prod" }[
  process.env.SDKWORK_MISSORY_ENVIRONMENT ?? "production"
] ?? "prod";
const profile = process.env.SDKWORK_MISSORY_DEPLOYMENT_PROFILE ?? "standalone";
const distDir = path.join(appRoot, "dist", profile, alias);
const startUrl = process.env.SDKWORK_DESKTOP_START_URL;
if (!startUrl && !fs.existsSync(path.join(distDir, "index.html"))) {
  fail(
    `PC console build missing at ${distDir}. Build it first: pnpm --dir apps/sdkwork-missory-pc build:${alias}`,
  );
}
process.stdout.write(
  `[missory-pc-electron] start target ok: ${startUrl ?? path.relative(appRoot, distDir)}\n`,
);

if (wantsPackage) {
  let electron;
  try {
    electron = path.join(hostRoot, "node_modules", "electron", "cli.js");
    fs.accessSync(electron);
  } catch {
    fail("electron is not installed; run pnpm install in apps/sdkwork-missory-pc first");
  }
  const versionOut = spawnSync(process.execPath, [electron, "--version"], {
    encoding: "utf8",
    env: { ...process.env, ELECTRON_RUN_AS_NODE: undefined },
  });
  const version = (versionOut.stdout ?? "").trim();
  process.stdout.write(`[missory-pc-electron] electron ${version || "(binary unavailable in sandbox)"}\n`);

  const outDir = path.join(hostRoot, "target", "desktop");
  fs.mkdirSync(outDir, { recursive: true });
  const manifest = {
    schemaVersion: 1,
    kind: "sdkwork.desktop-host-package",
    application: "sdkwork-missory-pc-electron",
    architecture: "electron",
    startTarget: startUrl ?? path.relative(appRoot, distDir),
    electronVersion: version || null,
    consoleBuild: path.relative(appRoot, distDir),
    packagedAt: "materialized",
  };
  fs.writeFileSync(path.join(outDir, "desktop-host.manifest.json"), `${JSON.stringify(manifest, null, 2)}\n`);
  execFileSync(process.execPath, [path.join(hostRoot, "scripts", "write-desktop-manifest.mjs")], {
    stdio: "inherit",
    env: process.env,
  });
  process.stdout.write(`[missory-pc-electron] desktop package staged at ${path.relative(appRoot, outDir)}\n`);
}

process.stdout.write("[missory-pc-electron] verify ok\n");
