import { resolveViteEnvironment } from "../../../sdkwork-specs/tools/vite-runtime-profile.mjs";
import { resolveBrowserDistOutDir } from "../../../sdkwork-specs/tools/browser-dist-layout.mjs";
import { buildBrowserDevRuntimeEnvDocument } from "../../../sdkwork-specs/tools/browser-runtime-env.mjs";
import { createBrowserRuntimeEnvVitePlugin } from "../../../sdkwork-specs/tools/browser-runtime-env-vite.mjs";

import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

const RUNTIME_ENV_DOCUMENT_PATH = "/runtime-env.json";

function resolveDevAppApiBaseUrl() {
  return process.env.SDKWORK_MISSORY_APPLICATION_PUBLIC_HTTP_URL ?? "http://127.0.0.1:8460";
}

// Serve-only dev runtime document. public/runtime-env.json is a per-build deploy
// artifact (prebuild writes it). Phase-1 has no webserver front in dev, so the
// served document overrides the canonical same-origin bases with the standalone
// gateway origin (cross-origin); the canonical helper still owns identity fields.
function missoryRuntimeEnvDocumentPlugin(mode: string) {
  return createBrowserRuntimeEnvVitePlugin({
    name: "missory-runtime-env-document",
    path: RUNTIME_ENV_DOCUMENT_PATH,
    resolveServeDocument: () => {
      const base = resolveDevAppApiBaseUrl();
      const document = {
        ...buildBrowserDevRuntimeEnvDocument({ profileId: mode }),
        browserOriginMode: "cross-origin",
        appApiBaseUrl: base,
        backendApiBaseUrl: base,
        openApiBaseUrl: base,
      };
      return JSON.stringify(document);
    },
  });
}

export default defineConfig(({ mode }: { mode: string }) => ({
  plugins: [missoryRuntimeEnvDocumentPlugin(mode), tailwindcss(), react()],
  resolve: {
    dedupe: ["react", "react-dom", "react-router", "react-router-dom"],
  },
  server: { host: "127.0.0.1", port: 5197 },
  preview: { host: "127.0.0.1", port: 5917 },
  build: {
    outDir: resolveBrowserDistOutDir(resolveViteEnvironment(mode, process.env)),
    sourcemap: false,
    target: "es2022",
  },
}));
