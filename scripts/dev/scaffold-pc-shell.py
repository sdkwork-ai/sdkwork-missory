#!/usr/bin/env python3
"""Scaffolder for the missory PC commons/shell/people/memories/assistant packages."""
import json
import os

BASE = "apps/sdkwork-missory-pc/packages"

def w(path, content):
    full = os.path.join(BASE, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8", newline="\n") as f:
        f.write(content)

def pkg_json(name, deps):
    return json.dumps({
        "name": name, "version": "0.1.0", "private": True, "type": "module",
        "main": "./src/index.ts", "module": "./src/index.ts", "types": "./src/index.ts",
        "exports": {
            ".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"},
            "./styles.css": "./src/styles.css",
        } if name.endswith("commons") else
        {".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"}},
        "dependencies": deps,
    }, indent=2, ensure_ascii=False) + "\n"

REACT = {"react": "catalog:", "react-dom": "catalog:", "react-router-dom": "catalog:"}

# ---------------- pc-commons ----------------
w("sdkwork-missory-pc-commons/package.json", pkg_json("@sdkwork/missory-pc-commons", REACT))
w("sdkwork-missory-pc-commons/src/index.ts", '''export { SectionCard } from "./components/SectionCard.tsx";
export { LoadingState } from "./components/LoadingState.tsx";
export { EmptyState } from "./components/EmptyState.tsx";
export { ErrorState } from "./components/ErrorState.tsx";
export { Badge } from "./components/Badge.tsx";
export { TextField } from "./components/TextField.tsx";
export { formatRelativeDays, truncate } from "./utils/format.ts";
export { memoryTypeLabel, relationshipTypeLabel, importanceLabel } from "./utils/labels.ts";
''')

w("sdkwork-missory-pc-commons/src/styles.css", '''.sdk-card {
  border: 1px solid var(--sdk-color-border);
  border-radius: 12px;
  background: var(--sdk-color-surface);
  padding: 16px;
}

.sdk-muted {
  color: var(--sdk-color-text-muted);
}

.sdk-input {
  width: 100%;
  border: 1px solid var(--sdk-color-border);
  border-radius: 8px;
  background: var(--sdk-color-surface);
  color: var(--sdk-color-text);
  padding: 8px 10px;
  font-size: 14px;
}

.sdk-button {
  border: 1px solid var(--sdk-color-border);
  border-radius: 8px;
  padding: 8px 14px;
  font-size: 14px;
  cursor: pointer;
  background: var(--sdk-color-surface);
  color: var(--sdk-color-text);
}

.sdk-button-primary {
  background: var(--sdk-color-primary);
  border-color: var(--sdk-color-primary);
  color: var(--sdk-color-primary-contrast);
}

.sdk-button:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

.sdk-badge {
  display: inline-flex;
  align-items: center;
  border-radius: 999px;
  padding: 2px 10px;
  font-size: 12px;
  border: 1px solid var(--sdk-color-border);
  background: var(--sdk-color-surface-muted);
  color: var(--sdk-color-text-muted);
}
''')

w("sdkwork-missory-pc-commons/src/components/SectionCard.tsx", '''import type { ReactNode } from "react";

export function SectionCard({ title, action, children }: {
  title: string;
  action?: ReactNode;
  children: ReactNode;
}) {
  return (
    <section className="sdk-card" style={{ display: "grid", gap: 12 }}>
      <header style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
        <h3 style={{ margin: 0, fontSize: 15 }}>{title}</h3>
        {action}
      </header>
      {children}
    </section>
  );
}
''')

w("sdkwork-missory-pc-commons/src/components/LoadingState.tsx", '''export function LoadingState({ label = "加载中…" }: { label?: string }) {
  return (
    <div className="sdk-muted" style={{ padding: 16, textAlign: "center" }} role="status">
      {label}
    </div>
  );
}
''')

w("sdkwork-missory-pc-commons/src/components/EmptyState.tsx", '''export function EmptyState({ title, hint }: { title: string; hint?: string }) {
  return (
    <div style={{ padding: 20, textAlign: "center", display: "grid", gap: 6 }}>
      <div style={{ fontWeight: 600 }}>{title}</div>
      {hint ? <div className="sdk-muted" style={{ fontSize: 13 }}>{hint}</div> : null}
    </div>
  );
}
''')

w("sdkwork-missory-pc-commons/src/components/ErrorState.tsx", '''export function ErrorState({ message, onRetry }: { message: string; onRetry?: () => void }) {
  return (
    <div
      role="alert"
      style={{
        padding: 16,
        border: "1px solid var(--sdk-color-danger)",
        borderRadius: 10,
        display: "grid",
        gap: 8,
      }}
    >
      <div style={{ color: "var(--sdk-color-danger)" }}>{message}</div>
      {onRetry ? (
        <button type="button" className="sdk-button" onClick={onRetry}>
          重试
        </button>
      ) : null}
    </div>
  );
}
''')

w("sdkwork-missory-pc-commons/src/components/Badge.tsx", '''import type { ReactNode } from "react";

export function Badge({ children }: { children: ReactNode }) {
  return <span className="sdk-badge">{children}</span>;
}
''')

w("sdkwork-missory-pc-commons/src/components/TextField.tsx", '''export function TextField({ label, value, onChange, placeholder, type = "text" }: {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: string;
}) {
  return (
    <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
      <span className="sdk-muted">{label}</span>
      <input
        className="sdk-input"
        type={type}
        value={value}
        placeholder={placeholder}
        onChange={(event) => onChange(event.target.value)}
      />
    </label>
  );
}
''')

w("sdkwork-missory-pc-commons/src/utils/format.ts", '''export function formatRelativeDays(days: number | undefined | null): string {
  if (days === undefined || days === null) return "暂无记录";
  if (days <= 0) return "今天";
  if (days < 30) return `${days} 天前`;
  if (days < 365) return `${Math.floor(days / 30)} 个月前`;
  return `${Math.floor(days / 365)} 年前`;
}

export function truncate(text: string, max: number): string {
  return text.length <= max ? text : `${text.slice(0, max)}…`;
}

export function formatDateTime(value: string | undefined | null): string {
  if (!value) return "—";
  return value.slice(0, 10);
}
''')

w("sdkwork-missory-pc-commons/src/utils/labels.ts", '''export function relationshipTypeLabel(value: string): string {
  const map: Record<string, string> = {
    family: "家人", friend: "朋友", classmate: "同学", colleague: "同事",
    client: "客户", partner: "合作伙伴", teacher: "老师", student: "学生",
    neighbor: "邻居", spouse: "伴侣", other: "其他",
  };
  return map[value] ?? value;
}

export function memoryTypeLabel(value: string): string {
  const map: Record<string, string> = {
    semantic: "事实", episodic: "经历", temporal: "近况",
    relationship: "关系", preference: "偏好", commitment: "承诺",
  };
  return map[value] ?? value;
}

export function importanceLabel(value: string | undefined): string {
  if (!value) return "普通";
  const map: Record<string, string> = { low: "低", normal: "普通", high: "重要", core: "核心" };
  return map[value] ?? value;
}
''')

w("sdkwork-missory-pc-commons/README.md", '''# @sdkwork/missory-pc-commons

Shared PC presentation primitives, formatting helpers, and the single feature
stylesheet consumed by the app shell. No SDK imports live here.
''')

# ---------------- pc-shell ----------------
w("sdkwork-missory-pc-shell/package.json", pkg_json("@sdkwork/missory-pc-shell", {
    "@sdkwork/missory-pc-core": "workspace:*",
    "@sdkwork/missory-pc-commons": "workspace:*",
    **REACT,
}))

w("sdkwork-missory-pc-shell/src/index.ts", '''export { MissoryAppShell } from "./AppShell.tsx";
export { HomeScreen } from "./screens/HomeScreen.tsx";
export { navigationModules } from "./navigation/modules.ts";
''')

w("sdkwork-missory-pc-shell/src/navigation/modules.ts", '''export interface NavigationModule {
  id: string;
  path: string;
  label: string;
}

export const navigationModules: NavigationModule[] = [
  { id: "home", path: "/", label: "首页" },
  { id: "people", path: "/people", label: "人物" },
  { id: "memories", path: "/memories", label: "记忆" },
  { id: "assistant", path: "/assistant", label: "AI" },
];
''')

w("sdkwork-missory-pc-shell/src/AppShell.tsx", '''import { NavLink, Outlet } from "react-router-dom";

import { navigationModules } from "./navigation/modules.ts";
import type { BootstrappedMissoryPcRuntime } from "../../src/bootstrap/runtime";

export function MissoryAppShell({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
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
''')

w("sdkwork-missory-pc-shell/src/screens/HomeScreen.tsx", '''import { useCallback, useEffect, useState } from "react";
import { Link } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard,
  formatRelativeDays, truncate,
} from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { BootstrappedMissoryPcRuntime } from "../../src/bootstrap/runtime";

interface HomeModel {
  loading: boolean;
  error: string | null;
  reminders: { reminderId: string; personName: string; title: string; type: string }[];
  memories: { id: string; content: string; createdAt: string }[];
  persons: { id: string; displayName: string; lastContactedAt: string | null }[];
}

export function HomeScreen({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
  const [model, setModel] = useState<HomeModel>({
    loading: true, error: null, reminders: [], memories: [], persons: [],
  });

  const refresh = useCallback(async () => {
    setModel((previous) => ({ ...previous, loading: true, error: null }));
    try {
      const digest = await runtime.home.today();
      setModel({
        loading: false,
        error: null,
        reminders: digest.todayReminders.map((reminder) => ({
          reminderId: reminder.reminderId,
          personName: reminder.personName,
          title: reminder.title,
          type: reminder.reminderType,
        })),
        memories: digest.recentMemories.map((memory) => ({
          id: String(memory.id),
          content: memory.content,
          createdAt: memory.createdAt,
        })),
        persons: digest.recentPersons.map((person) => ({
          id: String(person.id),
          displayName: person.displayName,
          lastContactedAt: person.lastContactedAt ?? null,
        })),
      });
    } catch (error) {
      setModel((previous) => ({
        ...previous,
        loading: false,
        error: normalizeClientError(error).message,
      }));
    }
  }, [runtime]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const dismiss = useCallback(async (reminderId: string) => {
    await runtime.home.dismissReminder(reminderId);
    await refresh();
  }, [runtime, refresh]);

  if (model.loading) return <LoadingState />;
  if (model.error) return <ErrorState message={model.error} onRetry={() => void refresh()} />;

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>今天，有谁值得你想起？</h2>
      <SectionCard title="今日关系">
        {model.reminders.length === 0 ? (
          <EmptyState title="暂无提醒" hint="创建人物并设定联系周期后，这里会提醒你。" />
        ) : (
          <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
            {model.reminders.map((reminder) => (
              <li
                key={reminder.reminderId}
                style={{ display: "flex", alignItems: "center", gap: 10, justifyContent: "space-between" }}
              >
                <span>
                  <strong>{reminder.personName}</strong>
                  <span className="sdk-muted"> · {reminder.title}</span>{" "}
                  <Badge>{reminder.type}</Badge>
                </span>
                <button
                  type="button"
                  className="sdk-button"
                  onClick={() => void dismiss(reminder.reminderId)}
                >
                  忽略
                </button>
              </li>
            ))}
          </ul>
        )}
      </SectionCard>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <SectionCard title="最近记忆">
          {model.memories.length === 0 ? (
            <EmptyState title="还没有记忆" hint="在「记忆」页记录第一条吧。" />
          ) : (
            <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
              {model.memories.map((memory) => (
                <li key={memory.id}>
                  <span>{truncate(memory.content, 40)}</span>
                  <span className="sdk-muted" style={{ fontSize: 12 }}> · {formatRelativeDays(null)}</span>
                </li>
              ))}
            </ul>
          )}
        </SectionCard>
        <SectionCard title="最近人物">
          {model.persons.length === 0 ? (
            <EmptyState title="还没有人物" hint="去「人物」页创建第一个重要的人。" />
          ) : (
            <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
              {model.persons.map((person) => (
                <li key={person.id} style={{ display: "flex", justifyContent: "space-between" }}>
                  <Link to={`/people/${person.id}`} style={{ color: "var(--sdk-color-primary)" }}>
                    {person.displayName}
                  </Link>
                  <span className="sdk-muted" style={{ fontSize: 12 }}>
                    {formatRelativeDays(null)}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </SectionCard>
      </div>
    </div>
  );
}
''')

w("sdkwork-missory-pc-shell/README.md", '''# @sdkwork/missory-pc-shell

PC app shell: navigation modules, layout, and the home digest screen. Screens
consume services from `@sdkwork/missory-pc-core` through the bootstrapped
runtime; the shell never constructs SDK clients.
''')

print("commons + shell written")
