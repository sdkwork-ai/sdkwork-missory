/**
 * Repository architecture contract test (node --test):
 * verifies the standard-architecture invariants for sdkwork-missory that the
 * shared validators do not cover — crate naming families, layering direction,
 * surface prefix, and the Fact ≠ Inference contract surfacing.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const read = (relative) => fs.readFileSync(path.join(root, relative), 'utf8');
const exists = (relative) => fs.existsSync(path.join(root, relative));

test('workspace declares the standard crate families', () => {
  const cargo = read('Cargo.toml');
  for (const crate of [
    'crates/sdkwork-missory-contract',
    'crates/sdkwork-missory-spi',
    'crates/sdkwork-communication-missory-service',
    'crates/sdkwork-routes-missory-app-api',
    'crates/sdkwork-api-missory-assembly',
    'crates/sdkwork-api-missory-standalone-gateway',
    'crates/sdkwork-missory-test-support',
    'plugins/sdkwork-missory-plugin-store-memory',
  ]) {
    assert.ok(cargo.includes(`"${crate}"`), `workspace member missing: ${crate}`);
  }
});

test('route crate mounts only the locked app-api prefix', () => {
  const lib = read('crates/sdkwork-routes-missory-app-api/src/lib.rs');
  assert.match(lib, /APP_API_PREFIX:\s*&str\s*=\s*"\/app\/v3\/api"/u);
  const paths = read('crates/sdkwork-routes-missory-app-api/src/paths.rs');
  for (const match of paths.matchAll(/"(\/[^"]+)"/gu)) {
    assert.ok(match[1].startsWith('/app/v3/api/missory/'), `route outside app-api prefix: ${match[1]}`);
  }
});

test('route crate depends on contract, never on store adapters', () => {
  const cargo = read('crates/sdkwork-routes-missory-app-api/Cargo.toml');
  const dependenciesSection = cargo.split(/\n\[/u).find(
    (section) => section.startsWith('dependencies]'),
  );
  assert.ok(dependenciesSection, '[dependencies] section missing');
  assert.ok(dependenciesSection.includes('sdkwork-missory-contract.workspace = true'));
  // Store adapters may appear only as dev-dependencies for router smoke tests
  // (fleet practice: sdkwork-routes-memory-app-api does the same).
  assert.ok(
    !dependenciesSection.includes('sdkwork-missory-plugin-store-memory'),
    'route crate [dependencies] must not include store adapters',
  );
  assert.ok(!dependenciesSection.includes('sqlx'), 'route crate must not depend on SQL drivers');
});

test('service crate depends on SPI ports, never on HTTP frameworks', () => {
  const cargo = read('crates/sdkwork-communication-missory-service/Cargo.toml');
  assert.ok(cargo.includes('sdkwork-missory-spi.workspace = true'));
  for (const forbidden of ['axum', 'hyper', 'tower-http']) {
    assert.ok(!cargo.includes(`${forbidden}.workspace`), `service crate must not depend on ${forbidden}`);
  }
});

test('store adapter never depends on HTTP frameworks', () => {
  const cargo = read('plugins/sdkwork-missory-plugin-store-memory/Cargo.toml');
  for (const forbidden of ['axum', 'hyper', 'tower', 'serde_json']) {
    assert.ok(!cargo.includes(`${forbidden}.workspace`), `store adapter must not depend on ${forbidden}`);
  }
});

test('assembly manifest and route manifest are materialized and consistent', () => {
  assert.ok(exists('crates/sdkwork-api-missory-assembly/assembly-manifest.json'));
  const assembly = JSON.parse(read('crates/sdkwork-api-missory-assembly/assembly-manifest.json'));
  assert.equal(assembly.applicationCode, 'missory');
  assert.equal(assembly.apiMode, 'served');
  const routeManifest = JSON.parse(
    read('sdks/_route-manifests/app-api/sdkwork-routes-missory-app-api.route-manifest.json'),
  );
  assert.equal(routeManifest.prefix, '/app/v3/api');
  assert.ok(routeManifest.routes.length >= 30, 'app-api surface should expose the P0 routes');
});

test('openapi authority keeps int64 ids as strings', () => {
  const openapi = read('apis/app-api/communication/missory-app-api.openapi.json');
  const doc = JSON.parse(openapi);
  const int64 = doc.components.schemas.Int64Id;
  assert.equal(int64.type, 'string');
  assert.equal(int64.format, 'int64');
  assert.equal(int64['x-sdkwork-int64-string'], true);
});

test('every authored crate owns a component spec', () => {
  for (const crate of [
    'crates/sdkwork-missory-contract',
    'crates/sdkwork-missory-spi',
    'crates/sdkwork-communication-missory-service',
    'crates/sdkwork-routes-missory-app-api',
    'crates/sdkwork-api-missory-assembly',
    'crates/sdkwork-api-missory-standalone-gateway',
    'crates/sdkwork-missory-test-support',
    'plugins/sdkwork-missory-plugin-store-memory',
  ]) {
    assert.ok(exists(`${crate}/specs/component.spec.json`), `component spec missing: ${crate}`);
  }
});
