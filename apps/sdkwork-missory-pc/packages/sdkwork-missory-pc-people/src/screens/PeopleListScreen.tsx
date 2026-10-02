import { useCallback, useEffect, useState } from "react";
import { Link } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard, TextField,
  relationshipTypeLabel,
} from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryPerson } from "@sdkwork/missory-pc-core";
import type { MissoryPcRuntime } from "@sdkwork/missory-pc-core";

export function PeopleListScreen({ runtime }: { runtime: MissoryPcRuntime }) {
  const [keyword, setKeyword] = useState("");
  const [items, setItems] = useState<MissoryPerson[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const [displayName, setDisplayName] = useState("");

  const refresh = useCallback(async (q: string) => {
    setLoading(true);
    setError(null);
    try {
      const page = await runtime.people.list(q ? { q, pageSize: 50 } : { pageSize: 50 });
      setItems(page.items);
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setLoading(false);
    }
  }, [runtime]);

  useEffect(() => {
    void refresh("");
  }, [refresh]);

  const createPerson = useCallback(async () => {
    if (!displayName.trim()) return;
    setCreating(true);
    try {
      const person = await runtime.people.create({
        displayName: displayName.trim(),
        relationshipTypes: ["friend"],
      });
      setDisplayName("");
      await refresh(keyword);
      void person;
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setCreating(false);
    }
  }, [runtime, displayName, keyword, refresh]);

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>人物</h2>
      <SectionCard title="创建人物">
        <div style={{ display: "flex", gap: 8, alignItems: "end" }}>
          <TextField label="姓名" value={displayName} onChange={setDisplayName} placeholder="例如：李明" />
          <button
            type="button"
            className="sdk-button sdk-button-primary"
            disabled={creating || !displayName.trim()}
            onClick={() => void createPerson()}
          >
            创建
          </button>
        </div>
      </SectionCard>
      <SectionCard
        title={`全部人物 (${items.length})`}
        action={
          <div style={{ display: "flex", gap: 8 }}>
            <input
              className="sdk-input"
              style={{ width: 200 }}
              value={keyword}
              placeholder="搜索姓名/标签/公司"
              onChange={(event) => setKeyword(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") void refresh(keyword);
              }}
            />
            <button type="button" className="sdk-button" onClick={() => void refresh(keyword)}>
              搜索
            </button>
          </div>
        }
      >
        {loading ? <LoadingState /> : null}
        {!loading && error ? <ErrorState message={error} onRetry={() => void refresh(keyword)} /> : null}
        {!loading && !error && items.length === 0 ? (
          <EmptyState title="还没有人物" hint="创建第一个重要的人。" />
        ) : null}
        {!loading && !error && items.length > 0 ? (
          <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
            {items.map((person) => (
              <li
                key={String(person.id)}
                style={{
                  display: "flex", justifyContent: "space-between", alignItems: "center",
                  borderBottom: "1px solid var(--sdk-color-border)", paddingBottom: 8,
                }}
              >
                <Link
                  to={`/people/${String(person.id)}`}
                  style={{ color: "var(--sdk-color-primary)", textDecoration: "none", fontWeight: 600 }}
                >
                  {person.displayName}
                </Link>
                <span style={{ display: "flex", gap: 6 }}>
                  {(person.tags ?? []).slice(0, 2).map((tag) => (
                    <Badge key={tag}>{tag}</Badge>
                  ))}
                  {person.title ? <span className="sdk-muted" style={{ fontSize: 12 }}>{person.title}</span> : null}
                </span>
              </li>
            ))}
          </ul>
        ) : null}
      </SectionCard>
      <p className="sdk-muted" style={{ fontSize: 12, margin: 0 }}>
        关系类型（{relationshipTypeLabel("friend")} 等）可在人物详情中维护。
      </p>
    </div>
  );
}
