#!/usr/bin/env node
/**
 * Materialize Missory API contracts from the authored OpenAPI authority:
 *  - sdks/_route-manifests/app-api/sdkwork-routes-missory-app-api.route-manifest.json
 *
 * The route crate's Rust manifest (http route wiring) is compared against this
 * manifest by tests/contracts; regenerate after changing the OpenAPI authority.
 * Authority: API_ASSEMBLY_SPEC.md, SDK_WORKSPACE_GENERATION_SPEC.md.
 */
import fs from 'node:fs';
import path from 'node:path';
import { parseArgs } from 'node:util';

const args = process.argv.slice(2);
const getArg = (name, fallback = null) => {
  const index = args.findIndex((item) => item === `--${name}`);
  if (index < 0) return fallback;
  const next = args[index + 1];
  return next && !next.startsWith('--') ? next : fallback;
};

const root = path.resolve(getArg('root', process.cwd()));
const authorityPath = path.join(root, 'apis/app-api/communication/missory-app-api.openapi.json');
const outDir = path.join(root, 'sdks/_route-manifests/app-api');
const outFile = path.join(outDir, 'sdkwork-routes-missory-app-api.route-manifest.json');

const openapi = JSON.parse(fs.readFileSync(authorityPath, 'utf8'));
const routes = [];
for (const [pathKey, pathItem] of Object.entries(openapi.paths ?? {})) {
  for (const [method, operation] of Object.entries(pathItem ?? {})) {
    if (!['get', 'post', 'put', 'patch', 'delete'].includes(method)) continue;
    routes.push({
      method: method.toUpperCase(),
      path: pathKey,
      operationId: operation.operationId,
      tags: operation.tags ?? ['missory'],
    });
  }
}

const missoryRoutes = routes.filter((route) => route.path.startsWith('/app/v3/api/missory/'));

const manifest = {
  schemaVersion: 1,
  kind: 'sdkwork.route.manifest',
  packageName: 'sdkwork-routes-missory-app-api',
  surface: 'app-api',
  owner: 'sdkwork-missory',
  domain: 'communication',
  capability: 'missory',
  apiAuthority: 'sdkwork-missory-app-api',
  sdkFamily: 'sdkwork-missory-app-sdk',
  prefix: '/app/v3/api',
  source: {
    crateRoot: 'crates/sdkwork-routes-missory-app-api',
    crateImport: 'sdkwork_routes_missory_app_api',
    openApiAuthority: 'apis/app-api/communication/missory-app-api.openapi.json',
  },
  routes: missoryRoutes.map((route) => ({
    method: route.method,
    path: route.path,
    operationId: route.operationId,
    tags: ['missory'],
    auth: { mode: 'header-context', required: true },
    handler: { module: 'crate::routes', name: null },
    ownership: { owner: 'sdkwork-missory', apiAuthority: 'sdkwork-missory-app-api' },
    requestContext: 'MissoryRequestContext',
    apiSurface: 'app-api',
  })),
};

fs.mkdirSync(outDir, { recursive: true });
fs.writeFileSync(outFile, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
process.stdout.write(
  `materialized ${outFile} with ${manifest.routes.length} routes\n`,
);
