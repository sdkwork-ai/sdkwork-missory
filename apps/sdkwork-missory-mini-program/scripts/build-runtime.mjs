#!/usr/bin/env node
// Bundles src/bootstrap/runtimeBundle.ts into src/runtime/runtime.js with the
// selected config/mini-program runtime-env values stamped in (MINI_PROGRAM_
// APP_ARCHITECTURE_SPEC section 5: one safe runtime module before upload).
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function option(argv, name, fallback) {
  const index = argv.indexOf(name);
  return index >= 0 ? argv[index + 1] : fallback;
}

const argv = process.argv.slice(2);
const deploymentProfile = option(argv, "--deployment-profile", "standalone");
const environment = option(argv, "--environment", "development");
const profileId = `${deploymentProfile}.${environment}`;

const envFile = path.join(appRoot, "config/mini-program", `runtime-env.${profileId}.json`);
if (!fs.existsSync(envFile)) {
  process.stderr.write(`runtime-env document missing: ${envFile}\n`);
  process.exit(2);
}
const doc = JSON.parse(fs.readFileSync(envFile, "utf8"));
for (const key of ["SDKWORK_ENVIRONMENT", "SDKWORK_DEPLOYMENT_PROFILE", "SDKWORK_PROFILE_ID", "SDKWORK_RUNTIME_TARGET"]) {
  if (!doc[key]) {
    process.stderr.write(`runtime-env document ${profileId} misses ${key}\n`);
    process.exit(2);
  }
}

const define = {
  "__SDKWORK_MISSORY_ENVIRONMENT__": JSON.stringify(doc.SDKWORK_ENVIRONMENT),
  "__SDKWORK_MISSORY_DEPLOYMENT_PROFILE__": JSON.stringify(doc.SDKWORK_DEPLOYMENT_PROFILE),
  "__SDKWORK_MISSORY_PROFILE_ID__": JSON.stringify(doc.SDKWORK_PROFILE_ID),
  "__SDKWORK_MISSORY_MP_APP_API_BASE_URL__": JSON.stringify(doc.SDKWORK_MISSORY_MP_APP_API_BASE_URL),
};

const outfile = path.join(appRoot, "src/runtime/runtime.js");
await build({
  entryPoints: [path.join(appRoot, "src/bootstrap/runtimeBundle.ts")],
  bundle: true,
  platform: "neutral",
  format: "cjs",
  target: "es2020",
  outfile,
  define,
  alias: {
    "@sdkwork/missory-mp-core": path.join(appRoot, "packages/sdkwork-missory-mp-core/src/index.ts"),
    "@sdkwork/missory-mp-host": path.join(appRoot, "packages/sdkwork-missory-mp-host/src/index.ts"),
    "@sdkwork/missory-mp-commons": path.join(appRoot, "packages/sdkwork-missory-mp-commons/src/index.ts"),
    "@sdkwork/missory-app-sdk": path.join(appRoot, "../../sdks/sdkwork-missory-app-sdk/sdkwork-missory-app-sdk-typescript/src/index.ts"),
  },
  external: ["@sdkwork/sdk-common", "@sdkwork/utils"],
});

fs.writeFileSync(
  path.join(appRoot, "src/runtime/build-manifest.json"),
  JSON.stringify({
    schemaVersion: 1,
    kind: "sdkwork.mini-program.runtime-manifest",
    application: "sdkwork-missory-mini-program",
    profileId,
    entry: "runtime.js",
    builtAt: "materialized",
  }, null, 2) + "\n",
);
process.stdout.write(`[sdkwork-missory-mp] runtime bundled for ${profileId} -> src/runtime/runtime.js\n`);
