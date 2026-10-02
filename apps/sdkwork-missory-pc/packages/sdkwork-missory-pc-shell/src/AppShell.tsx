import { useState } from "react";
import { NavLink, Outlet } from "react-router-dom";

import { navigationModules } from "./navigation/modules.ts";
import type { MissoryPcRuntime } from "@sdkwork/missory-pc-core";

export function MissoryAppShell({
  runtime,
  onSessionEnded,
}: {
  runtime: MissoryPcRuntime;
  onSessionEnded?: () => void;
}) {
  const [logoutError, setLogoutError] = useState<string | null>(null);
  const logout = () => {
    try {
      runtime.session.logout();
    } catch (cause) {
      setLogoutError(cause instanceof Error ? cause.message : String(cause));
      return;
    }
    setLogoutError(null);
    onSessionEnded?.();
  };
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
        <button
          type="button"
          onClick={logout}
          style={{
            marginTop: 12,
            padding: "6px 10px",
            borderRadius: 8,
            border: "1px solid var(--sdk-color-border)",
            background: "transparent",
            color: "var(--sdk-color-text)",
            cursor: "pointer",
          }}
        >
          退出登录
        </button>
        {logoutError ? (
          <div role="alert" style={{ fontSize: 12, color: "#b91c1c" }}>
            退出失败：{logoutError}
          </div>
        ) : null}
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
