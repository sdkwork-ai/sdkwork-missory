#!/usr/bin/env node
// Live-database login E2E for the standalone gateway (runbook:
// docs/architecture/tech/TECH_ARCHITECTURE.md section 8 item 4).
//
// Chain: db-migrate -> issue-bootstrap-token -> gateway boot (development,
// real database, dev bypass OFF) -> credential-entry registration ->
// password login -> dual-token business calls -> negative auth matrix ->
// cross-user isolation.
//
// Usage: pnpm run e2e:login
//   SDKWORK_DATABASE_URL overrides the target database (default: the
//   workspace development PostgreSQL built into sdkwork-database-config).
// ASCII-only payloads keep assertions immune to console code pages.

import { spawn, spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import process from "node:process";

const ROOT = join(import.meta.dirname, "..", "..");
const GATEWAY_BIN = join(
  ROOT,
  "target",
  "debug",
  `sdkwork-api-missory-standalone-gateway${process.platform === "win32" ? ".exe" : ""}`,
);
const BIND = "127.0.0.1:8461";
const BASE = `http://${BIND}`;
const DATABASE_URL =
  process.env.SDKWORK_DATABASE_URL ??
  "postgres://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev";
const BOOT_ENV = {
  ...process.env,
  SDKWORK_MISSORY_ENVIRONMENT: "development",
  SDKWORK_DATABASE_URL: DATABASE_URL,
};

const failures = [];
function check(label, ok, detail = "") {
  const mark = ok ? "PASS" : "FAIL";
  console.log(`[${mark}] ${label}${detail ? ` — ${detail}` : ""}`);
  if (!ok) failures.push(label);
}

function runGateway(args, timeoutMs = 120_000) {
  const result = spawnSync(GATEWAY_BIN, args, {
    env: BOOT_ENV,
    encoding: "utf8",
    timeout: timeoutMs,
    windowsHide: true,
  });
  return {
    status: result.status,
    stdout: result.stdout ?? "",
    stderr: result.stderr ?? "",
  };
}

async function waitForHealth(timeoutMs = 30_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`${BASE}/healthz`);
      if (response.ok) return true;
    } catch {
      // gateway not accepting yet
    }
    await new Promise((resolve) => setTimeout(resolve, 300));
  }
  return false;
}

async function postJson(path, body, headers = {}) {
  const response = await fetch(`${BASE}${path}`, {
    method: "POST",
    headers: { "content-type": "application/json", ...headers },
    body: JSON.stringify(body),
  });
  const text = await response.text();
  let json = null;
  try {
    json = text ? JSON.parse(text) : null;
  } catch {
    // non-JSON body is asserted by callers that expect it
  }
  return { status: response.status, contentType: response.headers.get("content-type") ?? "", json, text };
}

async function getJson(path, headers = {}) {
  const response = await fetch(`${BASE}${path}`, { headers });
  const text = await response.text();
  let json = null;
  try {
    json = text ? JSON.parse(text) : null;
  } catch {
    // non-JSON body is asserted by callers that expect it
  }
  return { status: response.status, contentType: response.headers.get("content-type") ?? "", json, text };
}

function dualTokens(session) {
  return { Authorization: `Bearer ${session.authToken}`, "Access-Token": session.accessToken };
}

function parseGatewayJson(stdout) {
  const stripped = stdout.replace(/\x1b\[[0-9;]*m/g, "");
  const lines = stripped
    .split(/\r?\n/)
    .filter((line) => !/^\s*\d{4}-\d{2}-\d{2}T/.test(line));
  return JSON.parse(lines.join("\n"));
}

async function main() {
  if (!existsSync(GATEWAY_BIN)) {
    console.error(`gateway binary missing: ${GATEWAY_BIN} — run cargo build first`);
    process.exit(2);
  }

  // 1. Migrations converge before any HTTP traffic.
  const migrate = runGateway(["db-migrate"]);
  check("db-migrate exits 0", migrate.status === 0, migrate.stderr.trim().split("\n").at(-1) ?? "");

  // 2. Bootstrap credential issuance (operator command).
  const bootstrap = runGateway(["issue-bootstrap-token", "--tenant", "100001", "--app", "sdkwork-missory"]);
  check("issue-bootstrap-token exits 0", bootstrap.status === 0);
  const credential = parseGatewayJson(bootstrap.stdout);
  check(
    "bootstrap credential carries dual tokens",
    Boolean(credential?.accessCredential && credential?.authToken),
  );
  const bootstrapHeaders = { "Access-Token": credential.accessCredential };

  // 3. Gateway boots against the real database with the dev bypass OFF.
  const gateway = spawn(GATEWAY_BIN, [], {
    env: { ...BOOT_ENV, SDKWORK_MISSORY_APPLICATION_PUBLIC_INGRESS_BIND: BIND },
    stdio: ["ignore", "ignore", "pipe"],
    windowsHide: true,
  });
  let gatewayLog = "";
  gateway.stderr.on("data", (chunk) => {
    gatewayLog += chunk.toString();
  });
  try {
    check("gateway /healthz becomes ready", await waitForHealth());

    // 4. User A registers through the credential-entry surface.
    const stamp = Date.now();
    const userA = { username: `e2e-a-${stamp}@missory.test`, password: "E2e-Passw0rd!" };
    const registration = await postJson(
      "/app/v3/api/auth/registrations",
      { ...userA, confirmPassword: userA.password, displayName: "E2E User A" },
      bootstrapHeaders,
    );
    check("registration accepted", registration.status === 201 || registration.status === 200,
      `status ${registration.status}`);
    check("registration auto-issues the session (data carries dual tokens + user id string)",
      Boolean(registration.json?.data?.authToken && registration.json?.data?.accessToken) &&
      /^\d+$/.test(String(registration.json?.data?.user?.id ?? "")));

    // 5. Password login issues the dual-token session.
    const login = await postJson(
      "/app/v3/api/auth/sessions",
      { grantType: "password", username: userA.username, password: userA.password },
      bootstrapHeaders,
    );
    check("login accepted", login.status === 200, `status ${login.status}`);
    const sessionA = login.json?.data?.item ?? login.json?.data;
    check("login issues authToken + accessToken",
      Boolean(sessionA?.authToken && sessionA?.accessToken));
    const headersA = dualTokens(sessionA);

    // 6. Business calls resolve the real principal.
    const profile = await getJson("/app/v3/api/missory/my_profile", headersA);
    check("my_profile 200 with envelope", profile.status === 200 && profile.json?.code === 0);
    check("my_profile resolves the real principal id (int64 as string)",
      /^\d+$/.test(String(profile.json?.data?.item?.userId ?? "")),
      `userId ${JSON.stringify(profile.json?.data?.item?.userId)}`);

    const created = await postJson(
      "/app/v3/api/missory/persons",
      { displayName: "E2E Person", city: "Hangzhou", tags: ["e2e"] },
      headersA,
    );
    check("person create 201 envelope", created.status === 201 && created.json?.code === 0);
    const personId = created.json?.data?.item?.id;
    check("person id is an int64 string", typeof personId === "string" && /^\d+$/.test(personId));

    const list = await getJson("/app/v3/api/missory/persons?page=1&page_size=20", headersA);
    check("person list 200 with items + pageInfo",
      list.status === 200 && Array.isArray(list.json?.data?.items) && Boolean(list.json?.data?.pageInfo));
    check("person list contains the created row",
      list.json?.data?.items?.some((row) => row.id === personId));

    const memory = await postJson(
      "/app/v3/api/missory/memories",
      { personId, type: "semantic", content: "E2E prefers window seats on flights." },
      headersA,
    );
    check("memory create 201 envelope", memory.status === 201 && memory.json?.code === 0);
    const userMemoryId = memory.json?.data?.item?.id;
    check("user-input memory is born confirmed", memory.json?.data?.item?.status === "confirmed");

    const extraction = await postJson(
      "/app/v3/api/missory/memories/extract",
      { personId, text: "E2E Person likes window seats. E2E Person is a colleague in Hangzhou." },
      headersA,
    );
    const candidates = extraction.json?.data?.items ?? [];
    check("memory extraction produces candidate items", extraction.status === 200 &&
      candidates.length > 0 && candidates.every((row) => row.status === "candidate"),
      `status ${extraction.status} items ${candidates.length} body ${extraction.text.slice(0, 200)}`);
    const candidateId = candidates[0]?.id;

    const confirmed = await postJson(`/app/v3/api/missory/memories/${candidateId}/confirm`, {}, headersA);
    check("candidate memory confirm accepted as command",
      confirmed.status === 200 && confirmed.json?.data?.accepted === true);
    const reconfirm = await postJson(`/app/v3/api/missory/memories/${userMemoryId}/confirm`, {}, headersA);
    check("confirming a non-candidate memory is rejected 409 problem+json",
      reconfirm.status === 409 && reconfirm.json?.status === 409 &&
      Number.isInteger(reconfirm.json?.code) && typeof reconfirm.json?.traceId === "string");

    const home = await getJson("/app/v3/api/missory/home/today", headersA);
    check("home today digest 200", home.status === 200 && home.json?.code === 0 &&
      Boolean(home.json?.data?.item));

    // Privacy export (PRD §9): whole-account document scoped to user A.
    const exported = await postJson("/app/v3/api/missory/data_exports", {}, headersA);
    const exportItem = exported.json?.data?.item;
    check("data export 201 envelope", exported.status === 201 && exported.json?.code === 0);
    check("export document carries profile + the created person + memories",
      typeof exportItem?.exportedAt === "string" &&
      /^\d+$/.test(String(exportItem?.profile?.userId ?? "")) &&
      Array.isArray(exportItem?.persons) &&
      exportItem.persons.some((row) => row.id === personId) &&
      Array.isArray(exportItem?.memories) && exportItem.memories.length > 0 &&
      Array.isArray(exportItem?.stories) && Array.isArray(exportItem?.reminders));

    const assistant = await postJson(
      "/app/v3/api/missory/assistant/query",
      { question: "What do I know about E2E Person?" },
      headersA,
    );
    check("assistant query answers with citation-bearing draft",
      assistant.status === 200 && assistant.json?.code === 0 &&
      Boolean(assistant.json?.data?.item));

    // 7. Negative auth matrix: fail closed without or with garbage tokens.
    const anonymous = await getJson("/app/v3/api/missory/persons");
    check("anonymous business call rejected 401", anonymous.status === 401);
    check("401 body is problem+json with numeric code and traceId",
      anonymous.contentType.includes("application/problem+json") &&
      Number.isInteger(anonymous.json?.code) && typeof anonymous.json?.traceId === "string");
    const garbage = await getJson("/app/v3/api/missory/persons", {
      Authorization: "Bearer not-a-real-token",
      "Access-Token": "also-garbage",
    });
    check("garbage dual tokens rejected 401", garbage.status === 401);

    // 8. Cross-user isolation: user B registers, logs in, sees none of A's rows.
    const userB = { username: `e2e-b-${stamp}@missory.test`, password: "E2e-Passw0rd!" };
    await postJson(
      "/app/v3/api/auth/registrations",
      { ...userB, confirmPassword: userB.password, displayName: "E2E User B" },
      bootstrapHeaders,
    );
    const loginB = await postJson(
      "/app/v3/api/auth/sessions",
      { grantType: "password", username: userB.username, password: userB.password },
      bootstrapHeaders,
    );
    const sessionB = loginB.json?.data?.item ?? loginB.json?.data;
    check("user B login issues a session", Boolean(sessionB?.authToken && sessionB?.accessToken));
    const listB = await getJson("/app/v3/api/missory/persons", dualTokens(sessionB));
    check("user B sees zero of user A's persons",
      listB.status === 200 && (listB.json?.data?.items ?? []).length === 0,
      `items ${(listB.json?.data?.items ?? []).length}`);
    const exportedB = await postJson("/app/v3/api/missory/data_exports", {}, dualTokens(sessionB));
    check("user B export excludes user A persons (isolation)",
      exportedB.status === 201 &&
      !(exportedB.json?.data?.item?.persons ?? []).some((row) => row.id === personId));
  } finally {
    gateway.kill();
    await new Promise((resolve) => setTimeout(resolve, 250));
    if (gateway.exitCode === null && !gateway.killed) {
      spawnSync("taskkill", ["/PID", String(gateway.pid), "/F", "/T"], { windowsHide: true });
    }
  }

  console.log("");
  if (failures.length > 0) {
    console.error(`login E2E FAILED (${failures.length} check(s)):`);
    for (const failure of failures) console.error(`  - ${failure}`);
    if (gatewayLog.trim()) console.error(`gateway log tail:\n${gatewayLog.trim().split("\n").slice(-3).join("\n")}`);
    process.exit(1);
  }
  console.log("login E2E passed: live-database dual-token chain + negative matrix + isolation");
}

main().catch((error) => {
  console.error("login E2E crashed:", error);
  process.exit(2);
});
