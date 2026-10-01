#!/usr/bin/env node
/**
 * Run the Missory standalone gateway with the standalone.development topology
 * profile env (etc/topology). Bypasses `sdkwork-app` orchestration for local
 * single-process iteration; profile values stay the single source of truth.
 */
import fs from 'node:fs';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { parseArgs } from 'node:util';

const args = process.argv.slice(2);
const getArg = (name, fallback = null) => {
  const index = args.findIndex((item) => item === `--${name}`);
  if (index < 0) return fallback;
  const next = args[index + 1];
  return next && !next.startsWith('--') ? next : fallback;
};

const root = path.resolve(getArg('root', process.cwd()));
const profile = getArg('profile', 'standalone.development');
const envFile = path.join(root, 'etc/topology', `${profile}.env`);

if (!fs.existsSync(envFile)) {
  process.stderr.write(`topology profile env not found: ${envFile}\n`);
  process.exit(2);
}

const env = { ...process.env };
for (const line of fs.readFileSync(envFile, 'utf8').split('\n')) {
  const trimmed = line.trim();
  if (!trimmed || trimmed.startsWith('#')) continue;
  const eq = trimmed.indexOf('=');
  if (eq > 0) env[trimmed.slice(0, eq)] = trimmed.slice(eq + 1);
}

const child = spawn(
  'cargo',
  ['run', '-p', 'sdkwork-api-missory-standalone-gateway'],
  { cwd: root, env, stdio: 'inherit', shell: process.platform === 'win32' },
);
child.on('exit', (code) => process.exit(code ?? 1));
