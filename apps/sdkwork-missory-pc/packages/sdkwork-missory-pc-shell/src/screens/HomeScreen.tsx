import { useCallback, useEffect, useState } from "react";
import { Link } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard,
  formatRelativeDays, truncate,
} from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryPcRuntime } from "@sdkwork/missory-pc-core";

interface HomeModel {
  loading: boolean;
  error: string | null;
  reminders: { reminderId: string; personName: string; title: string; type: string }[];
  memories: { id: string; content: string; createdAt: string }[];
  persons: { id: string; displayName: string; lastContactedAt: string | null }[];
}

export function HomeScreen({ runtime }: { runtime: MissoryPcRuntime }) {
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
          personName: reminder.personName ?? "",
          title: reminder.title,
          type: reminder.type,
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
