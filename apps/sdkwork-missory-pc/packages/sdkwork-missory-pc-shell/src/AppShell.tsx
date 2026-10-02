import { NavLink, Outlet } from "react-router-dom";

import { navigationModules } from "./navigation/modules.ts";
import type { MissoryPcRuntime } from "@sdkwork/missory-pc-core";

export function MissoryAppShell({ runtime }: { runtime: MissoryPcRuntime }) {
  return (
    <div style={{ display: "flex", minHeight: "100vh" }}>
      <aside
        style={{
          width: 200,
          borderRight: "1px solid var(--sdk-color-border)",
          padding: 16,
          display: "grid",
          gap: 8,
          alignContent: "start",
          background: "var(--sdk-color-surface)",
        }}
      >
        <div style={{ fontWeight: 700, fontSize: 16, marginBottom: 12 }}>念忆 · Missory</div>
        {navigationModules.map((module) => (
          <NavLink
            key={module.id}
            to={module.path}
            style={({ isActive }) => ({
              display: "block",
              padding: "8px 10px",
              borderRadius: 8,
              textDecoration: "none",
              color: isActive ? "var(--sdk-color-primary-contrast)" : "var(--sdk-color-text)",
              background: isActive ? "var(--sdk-color-primary)" : "transparent",
            })}
          >
            {module.label}
          </NavLink>
        ))}
        <div className="sdk-muted" style={{ fontSize: 11, marginTop: "auto" }}>
          {runtime.config.profileId} · {runtime.config.environment}
        </div>
      </aside>
      <main style={{ flex: 1, padding: 24, maxWidth: 980 }}>
        <Outlet />
      </main>
    </div>
  );
}
