// Missory desktop host (Electron). Loads the built PC web console from
// apps/sdkwork-missory-pc/dist (SDKWORK_DESKTOP_DIST_DIR) or a dev server URL
// (SDKWORK_DESKTOP_START_URL). No business logic lives here — the desktop host
// is a shell only (APP_PC_ARCHITECTURE_SPEC.md desktop-host rules).
import { app, BrowserWindow, shell } from "electron";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HOST_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const APP_ROOT = path.resolve(HOST_ROOT, "..", "..");
const REPO_ROOT = path.resolve(APP_ROOT, "..", "..");

function resolveStartTarget() {
  const startUrl = process.env.SDKWORK_DESKTOP_START_URL;
  if (startUrl) {
    return { url: startUrl };
  }
  const profile = process.env.SDKWORK_MISSORY_DEPLOYMENT_PROFILE ?? "standalone";
  const environment = process.env.SDKWORK_MISSORY_ENVIRONMENT ?? "production";
  const alias = { development: "dev", test: "test", staging: "staging", demo: "demo", production: "prod" }[environment] ?? environment;
  const distDir = process.env.SDKWORK_DESKTOP_DIST_DIR
    ?? path.join(APP_ROOT, "dist", profile, alias);
  const indexFile = path.join(distDir, "index.html");
  if (!fs.existsSync(indexFile)) {
    throw new Error(
      `PC console build not found at ${indexFile}; run pnpm build:${alias} (or build:${alias}:cloud) in apps/sdkwork-missory-pc first, or set SDKWORK_DESKTOP_START_URL.`,
    );
  }
  return { file: indexFile };
}

async function createWindow() {
  const target = resolveStartTarget();
  const window = new BrowserWindow({
    width: 1280,
    height: 860,
    minWidth: 960,
    minHeight: 640,
    title: "念忆 · Missory",
    backgroundColor: "#f8fafc",
    autoHideMenuBar: true,
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
    },
  });

  // External links open in the system browser; the desktop window stays on the console.
  window.webContents.setWindowOpenHandler(({ url }) => {
    void shell.openExternal(url);
    return { action: "deny" };
  });

  if (target.url) {
    await window.loadURL(target.url);
  } else {
    await window.loadFile(target.file);
  }
}

app.whenReady().then(async () => {
  await createWindow();
  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      void createWindow();
    }
  });
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

// Referenced for packaging evidence; keeps the repo root discoverable for the
// desktop host scripts without embedding business paths into the renderer.
void REPO_ROOT;
