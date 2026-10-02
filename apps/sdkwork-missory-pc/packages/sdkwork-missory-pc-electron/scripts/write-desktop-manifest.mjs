#!/usr/bin/env node
/**
 * Writes the packaged desktop host manifest (invoked by verify-desktop-host.mjs
 * --package). Split out so the packaging step stays a single responsibility.
 */
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const hostRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outDir = path.join(hostRoot, "target", "desktop");
const manifestPath = path.join(outDir, "desktop-host.manifest.json");
if (!fs.existsSync(manifestPath)) {
  process.stderr.write("desktop-host.manifest.json missing; run verify-desktop-host.mjs --package\n");
  process.exit(1);
}
const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
manifest.hostSources = ["src/main.mjs"];
manifest.consoleSurface = "apps/sdkwork-missory-pc (PC web console)";
manifest.wiredBy = "bin/apps-package.sh via pnpm --dir apps/sdkwork-missory-pc/packages/sdkwork-missory-pc-electron package";
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
process.stdout.write(`[missory-pc-electron] manifest finalized (${process.platform})\n`);
