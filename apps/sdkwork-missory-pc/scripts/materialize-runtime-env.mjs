// Invoked as `node scripts/materialize-runtime-env.mjs` (predev/prebuild and
// etc/sdkwork.deployment.config.json#materialization.command). Deliberately
// thin: every rule lives in the canonical shared runner.
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { materializeBrowserRuntimeEnv } from "../../../../sdkwork-specs/tools/build-browser-client.mjs";

const APP_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const REPOSITORY_ROOT = path.resolve(APP_ROOT, "..", "..");

function option(argv, name, fallback) {
  const index = argv.indexOf(name);
  return index >= 0 ? argv[index + 1] : fallback;
}

function main() {
  const argv = process.argv.slice(2);
  const deploymentProfile = option(argv, "--deployment-profile", "standalone");
  const environment = option(argv, "--environment", "development");
  const check = argv.includes("--check");

  materializeBrowserRuntimeEnv({
    appRoot: APP_ROOT,
    deploymentProfile,
    environment,
    repositoryRoot: REPOSITORY_ROOT,
    check,
  });

  console.log(
    `[sdkwork-missory-pc] runtime env ${check ? "verified" : "materialized"}: ${deploymentProfile}.${environment}`,
  );
}

try {
  main();
} catch (error) {
  console.error(`[sdkwork-missory-pc] ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
}
