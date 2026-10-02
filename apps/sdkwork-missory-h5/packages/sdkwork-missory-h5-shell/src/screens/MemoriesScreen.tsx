import { useCallback, useEffect, useState } from "react";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard,
  memoryTypeLabel, truncate,
} from "@sdkwork/missory-h5-commons";
import { normalizeClientError } from "@sdkwork/missory-h5-core";

import type { MissoryMemory, MissoryMemoryType, MissoryPerson } from "@sdkwork/missory-app-sdk";
import type { MissoryH5Runtime } from "@sdkwork/missory-h5-core";

const MEMORY_TYPES: MissoryMemoryType[] = [
  "semantic", "episodic", "temporal", "relationship", "preference", "commitment",
];

export function MemoriesScreen({ runtime }: { runtime: MissoryH5Runtime }) {
  const [items, setItems] = useState<MissoryMemory[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [keyword, setKeyword] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const [extractText, setExtractText] = useState("");
  const [extractNote, setExtractNote] = useState<string | null>(null);

  // 手动记录记忆
  const [persons, setPersons] = useState<MissoryPerson[]>([]);
  const [createPersonId, setCreatePersonId] = useState("");
  const [createType, setCreateType] = useState<MissoryMemoryType>("semantic");
  const [createContent, setCreateContent] = useState("");
  const [creating, setCreating] = useState(false);
  const [createNote, setCreateNote] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const page = await runtime.memories.list({
        pageSize: 50,
        q: keyword || undefined,
        status: statusFilter || undefined,
      });
      setItems(page.items);
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setLoading(false);
    }
  }, [runtime, keyword, statusFilter]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    let cancelled = false;
    runtime.people.list({ pageSize: 200 })
      .then((page) => {
        if (cancelled) return;
        setPersons(page.items);
        setCreatePersonId((previous) => previous || (page.items[0] ? String(page.items[0].id) : ""));
      })
      .catch(() => {
        // 人物列表加载失败不阻断记忆页，仅隐藏人物选择。
      });
    return () => {
      cancelled = true;
    };
  }, [runtime]);

  const confirm = useCallback(async (memoryId: string) => {
    await runtime.memories.confirm(memoryId);
    await refresh();
  }, [runtime, refresh]);

  const reject = useCallback(async (memoryId: string) => {
    await runtime.memories.reject(memoryId);
    await refresh();
  }, [runtime, refresh]);

  const remove = useCallback(async (memoryId: string) => {
    if (!window.confirm("确定删除这条记忆吗？删除后不可恢复。")) return;
    await runtime.memories.delete(memoryId);
    await refresh();
  }, [runtime, refresh]);

  const extract = useCallback(async () => {
    setExtractNote(null);
    if (!extractText.trim()) return;
    try {
      const page = await runtime.memories.extract({ text: extractText });
      setExtractNote(`已提取 ${page.items.length} 条候选记忆，请在列表中确认。`);
      setExtractText("");
      await refresh();
    } catch (cause) {
      setExtractNote(normalizeClientError(cause).message);
    }
  }, [runtime, extractText, refresh]);

  const createMemory = useCallback(async () => {
    if (!createPersonId || !createContent.trim() || creating) return;
    setCreating(true);
    setCreateNote(null);
    try {
      await runtime.memories.create({
        personId: createPersonId,
        type: createType,
        content: createContent.trim(),
      });
      setCreateContent("");
      setCreateNote("已记录为事实记忆。");
      await refresh();
    } catch (cause) {
      setCreateNote(normalizeClientError(cause).message);
    } finally {
      setCreating(false);
    }
  }, [runtime, createPersonId, createType, createContent, creating, refresh]);

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>记忆</h2>
      <SectionCard title="手动记录记忆">
        <div style={{ display: "grid", gap: 8 }}>
          <div style={{ display: "flex", gap: 8, alignItems: "end", flexWrap: "wrap" }}>
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">所属人物</span>
              <select
                className="sdk-input"
                style={{ width: 160 }}
                value={createPersonId}
                onChange={(event) => setCreatePersonId(event.target.value)}
              >
                {persons.length === 0 ? <option value="">（暂无人物）</option> : null}
                {persons.map((person) => (
                  <option key={String(person.id)} value={String(person.id)}>
                    {person.displayName}
                  </option>
                ))}
              </select>
            </label>
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">记忆类型</span>
              <select
                className="sdk-input"
                style={{ width: 130 }}
                value={createType}
                onChange={(event) => setCreateType(event.target.value as MissoryMemoryType)}
              >
                {MEMORY_TYPES.map((type) => (
                  <option key={type} value={type}>{memoryTypeLabel(type)}</option>
                ))}
              </select>
            </label>
          </div>
          <textarea
            className="sdk-input"
            rows={2}
            placeholder="例如：李明喜欢摄影。"
            value={createContent}
            onChange={(event) => setCreateContent(event.target.value)}
          />
          <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
            <button
              type="button"
              className="sdk-button sdk-button-primary"
              disabled={creating || !createPersonId || !createContent.trim()}
              onClick={() => void createMemory()}
            >
              记录
            </button>
            {createNote ? <span className="sdk-muted" style={{ fontSize: 13 }}>{createNote}</span> : null}
          </div>
        </div>
      </SectionCard>
      <SectionCard title="从文本提取候选记忆">
        <div style={{ display: "grid", gap: 8 }}>
          <textarea
            className="sdk-input"
            rows={3}
            placeholder="粘贴一段对话或随笔，例如：李明喜欢摄影。我答应帮李明介绍一个客户。"
            value={extractText}
            onChange={(event) => setExtractText(event.target.value)}
          />
          <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
            <button type="button" className="sdk-button sdk-button-primary" onClick={() => void extract()}>
              提取候选
            </button>
            {extractNote ? <span className="sdk-muted" style={{ fontSize: 13 }}>{extractNote}</span> : null}
          </div>
        </div>
      </SectionCard>
      <SectionCard
        title={`记忆列表 (${items.length})`}
        action={
          <div style={{ display: "flex", gap: 8 }}>
            <select
              className="sdk-input"
              style={{ width: 130 }}
              value={statusFilter}
              onChange={(event) => setStatusFilter(event.target.value)}
            >
              <option value="">全部状态</option>
              <option value="candidate">候选</option>
              <option value="confirmed">已确认</option>
              <option value="rejected">已拒绝</option>
            </select>
            <input
              className="sdk-input"
              style={{ width: 180 }}
              placeholder="搜索内容"
              value={keyword}
              onChange={(event) => setKeyword(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") void refresh();
              }}
            />
          </div>
        }
      >
        {loading ? <LoadingState /> : null}
        {!loading && error ? <ErrorState message={error} onRetry={() => void refresh()} /> : null}
        {!loading && !error && items.length === 0 ? (
          <EmptyState title="没有匹配的记忆" hint="手动记录或从文本提取。" />
        ) : null}
        {!loading && !error && items.length > 0 ? (
          <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 10 }}>
            {items.map((memory) => (
              <li
                key={String(memory.id)}
                style={{
                  display: "flex", justifyContent: "space-between", gap: 12,
                  borderBottom: "1px solid var(--sdk-color-border)", paddingBottom: 8,
                }}
              >
                <span>
                  <Badge>{memoryTypeLabel(memory.type)}</Badge>{" "}
                  <Badge>{memory.origin === "inference" ? "推断" : "事实"}</Badge>{" "}
                  <Badge>{memory.status === "candidate" ? "候选" : memory.status === "rejected" ? "已拒绝" : "已确认"}</Badge>{" "}
                  {truncate(memory.content, 60)}
                  {memory.confidence !== undefined && memory.confidence !== null ? (
                    <span className="sdk-muted" style={{ fontSize: 12 }}> · 置信 {memory.confidence}</span>
                  ) : null}
                </span>
                <span style={{ display: "flex", gap: 6, flexShrink: 0 }}>
                  {memory.status === "candidate" ? (
                    <>
                      <button
                        type="button"
                        className="sdk-button sdk-button-primary"
                        onClick={() => void confirm(String(memory.id))}
                      >
                        确认
                      </button>
                      <button type="button" className="sdk-button" onClick={() => void reject(String(memory.id))}>
                        拒绝
                      </button>
                    </>
                  ) : null}
                  <button
                    type="button"
                    className="sdk-button"
                    onClick={() => void remove(String(memory.id))}
                  >
                    删除
                  </button>
                </span>
              </li>
            ))}
          </ul>
        ) : null}
      </SectionCard>
      <p className="sdk-muted" style={{ fontSize: 12, margin: 0 }}>
        事实与推断严格分离（PRD §19）：推断只有确认后才成为已确认记忆。
      </p>
    </div>
  );
}
