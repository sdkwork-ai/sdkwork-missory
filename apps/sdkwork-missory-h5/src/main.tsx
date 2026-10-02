import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { installSessionExpiryBoundary } from "@sdkwork/missory-h5-core";
import { App } from "./App";
import { bootstrapMissoryH5Runtime } from "./bootstrap/runtime";
import "./index.css";
import "@sdkwork/missory-h5-commons/styles.css";

async function render() {
  const container = document.getElementById("root");
  if (!container) throw new Error("#root container is missing");
  installSessionExpiryBoundary();
  try {
    const runtime = await bootstrapMissoryH5Runtime();
    createRoot(container).render(
      <StrictMode>
        <App runtime={runtime} />
      </StrictMode>,
    );
  } catch (error) {
    createRoot(container).render(
      <div style={{ padding: 32, color: "var(--sdk-color-danger)" }}>
        念忆启动失败：{error instanceof Error ? error.message : String(error)}
      </div>,
    );
  }
}

void render();
