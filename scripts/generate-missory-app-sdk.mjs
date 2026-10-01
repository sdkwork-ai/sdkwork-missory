#!/usr/bin/env node
/**
 * Regenerate the sdkwork-missory-app-sdk family from the derived generator input.
 * Generated output under `generated/server-openapi` is generator-owned
 * (SDK_WORKSPACE_GENERATION_SPEC.md); run `pnpm sdk:check` after generation.
 */
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const sdkgen = path.join(root, '..', 'sdkwork-sdk-generator', 'bin', 'sdkgen.js');
const manifest = JSON.parse(
  fs.readFileSync(path.join(root, 'sdks/sdkwork-missory-app-sdk/sdk-manifest.json'), 'utf8'),
);

if (!fs.existsSync(sdkgen)) {
  process.stderr.write(`sdkgen not found at ${sdkgen}\n`);
  process.exit(2);
}
const generatorPackage = JSON.parse(
  fs.readFileSync(path.join(root, '..', 'sdkwork-sdk-generator', 'package.json'), 'utf8'),
);
if (generatorPackage.name !== '@sdkwork/sdk-generator') {
  process.stderr.write(`${sdkgen} is not @sdkwork/sdk-generator\n`);
  process.exit(2);
}

const input = path.join(root, 'sdks', manifest.sdkName, manifest.generationInputSpec);
const check = process.argv.includes('--check');

const languages = {
  typescript: {
    output: 'sdks/sdkwork-missory-app-sdk/sdkwork-missory-app-sdk-typescript/generated/server-openapi',
    packageName: '@sdkwork/missory-app-sdk',
    clientName: 'SdkworkMissoryAppClient',
  },
  dart: {
    output: 'sdks/sdkwork-missory-app-sdk/sdkwork-missory-app-sdk-dart/generated/server-openapi',
    packageName: 'sdkwork_missory_app_sdk',
    clientName: 'SdkworkMissoryAppClient',
  },
};

let failed = false;
for (const [language, spec] of Object.entries(languages)) {
  const args = [
    sdkgen, 'generate',
    '--input', input,
    '--output', path.join(root, spec.output),
    '--name', manifest.sdkName,
    '--type', 'app',
    '--language', language,
    '--sdk-name', manifest.sdkName,
    '--package-name', spec.packageName,
    '--client-name', spec.clientName,
    '--fixed-sdk-version', '0.1.0',
    '--standard-profile', manifest.standardProfile ?? 'sdkwork-v3',
    ...(check ? ['--dry-run'] : []),
  ];
  const result = spawnSync('node', args, { stdio: 'inherit' });
  if (result.status !== 0) {
    failed = true;
    process.stderr.write(`sdkgen ${language} failed with exit ${result.status}\n`);
  }
}
process.exit(failed ? 1 : 0);
