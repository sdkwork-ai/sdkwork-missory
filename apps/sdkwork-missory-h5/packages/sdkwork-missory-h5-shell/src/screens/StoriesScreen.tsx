import { useCallback, useEffect, useState } from "react";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard, TextField,
  formatDateTime, truncate,
} from "@sdkwork/missory-h5-commons";
import { normalizeClientError } from "@sdkwork/missory-h5-core";

import type { MissoryStory } from "@sdkwork/missory-app-sdk";
import type { MissoryH5Runtime } from "@sdkwork/missory-h5-core";

/** datetime-local 值（YYYY-MM-DDTHH:mm）补齐秒，满足 date-time wire 格式。 */
function toDateTimeWire(value: string): string | undefined {
  if (!value.trim()) return undefined;
  return value.length === 16 ? `${value}:00` : value;
}

export function StoriesScreen({ runtime }: { runtime: MissoryH5Runtime }) {
  const [items, setItems] = useState<MissoryStory[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const [title, setTitle] = useState("");
  const [startedAt, setStartedAt] = useState("");
  const [endedAt, setEndedAt] = useState("");
  const [location, setLocation] = useState("");
  const [openStory, setOpenStory] = useState<MissoryStory | null>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [busyStoryId, setBusyStoryId] = useState<string | null>(null);
  const [detailNote, setDetailNote] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const page = await runtime.stories.list({ pageSize: 50 });
      setItems(page.items);
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setLoading(false);
    }
  }, [runtime]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const openDetail = useCallback(async (story: MissoryStory) => {
    const storyId = String(story.id);
    if (openStory && String(openStory.id) === storyId) {
      setOpenStory(null);
      setDetailNote(null);
      return;
    }
    setOpenStory(story);
    setDetailNote(null);
    setDetailLoading(true);
    try {
      const fresh = await runtime.stories.retrieve(storyId);
      setOpenStory(fresh);
    } catch (cause) {
      setDetailNote(normalizeClientError(cause).message);
    } finally {
      setDetailLoading(false);
    }
  }, [runtime, openStory]);

  const createStory = useCallback(async () => {
    if (!title.trim() || creating) return;
    setCreating(true);
    try {
      await runtime.stories.create({
        title: title.trim(),
        ...(startedAt ? { startedAt: toDateTimeWire(startedAt) } : {}),
        ...(endedAt ? { endedAt: toDateTimeWire(endedAt) } : {}),
        ...(location.trim() ? { location: location.trim() } : {}),
      });
      setTitle("");
      setStartedAt("");
      setEndedAt("");
      setLocation("");
      await refresh();
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setCreating(false);
    }
  }, [runtime, title, startedAt, endedAt, location, creating, refresh]);

  const summarize = useCallback(async (storyId: string) => {
    setBusyStoryId(storyId);
    setDetailNote(null);
    try {
      const updated = await runtime.assistant.storySummary(storyId);
      setOpenStory((previous) => (previous && String(previous.id) === storyId ? updated : previous));
      setItems((previous) => previous.map((story) => (String(story.id) === storyId ? updated : story)));
      setDetailNote("AI 摘要已生成。");
    } catch (cause) {
      setDetailNote(normalizeClientError(cause).message);
    } finally {
      setBusyStoryId(null);
    }
  }, [runtime]);

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>故事</h2>
      <SectionCard title="记录一段故事">
        <div style={{ display: "grid", gap: 8 }}>
          <div style={{ display: "flex", gap: 8, alignItems: "end", flexWrap: "wrap" }}>
            <TextField label="标题（必填）" value={title} onChange={setTitle} placeholder="例如：和李明的青岛之行" />
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">开始时间（可选）</span>
              <input
                className="sdk-input"
                type="datetime-local"
                value={startedAt}
                onChange={(event) => setStartedAt(event.target.value)}
              />
            </label>
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">结束时间（可选）</span>
              <input
                className="sdk-input"
                type="datetime-local"
                value={endedAt}
                onChange={(event) => setEndedAt(event.target.value)}
              />
            </label>
            <TextField label="地点（可选）" value={location} onChange={setLocation} placeholder="例如：青岛" />
            <button
              type="button"
              className="sdk-button sdk-button-primary"
              disabled={creating || !title.trim()}
              onClick={() => void createStory()}
            >
              创建
            </button>
          </div>
        </div>
      </SectionCard>
      <SectionCard title={`故事列表 (${items.length})`}>
        {loading ? <LoadingState /> : null}
        {!loading && error ? <ErrorState message={error} onRetry={() => void refresh()} /> : null}
        {!loading && !error && items.length === 0 ? (
          <EmptyState title="还没有故事" hint="记录一段你和重要的人的共同经历。" />
        ) : null}
        {!loading && !error && items.length > 0 ? (
          <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 10 }}>
            {items.map((story) => {
              const storyId = String(story.id);
              const isOpen = openStory !== null && String(openStory.id) === storyId;
              return (
                <li
                  key={storyId}
                  style={{
                    display: "grid", gap: 8,
                    borderBottom: "1px solid var(--sdk-color-border)", paddingBottom: 8,
                  }}
                >
                  <button
                    type="button"
                    className="sdk-button"
                    style={{ justifyContent: "space-between", textAlign: "left", display: "flex" }}
                    onClick={() => void openDetail(story)}
                  >
                    <span>
                      <strong>{story.title}</strong>
                      {story.location ? <span className="sdk-muted"> · {story.location}</span> : null}
                      {isOpen ? null : story.summary ? (
                        <span className="sdk-muted"> · {truncate(story.summary, 30)}</span>
                      ) : null}
                    </span>
                    <span className="sdk-muted" style={{ fontSize: 12 }}>
                      {formatDateTime(story.startedAt)} ~ {formatDateTime(story.endedAt)}
                    </span>
                  </button>
                  {isOpen ? (
                    <div style={{ display: "grid", gap: 8, padding: "0 4px" }}>
                      {detailLoading ? <LoadingState /> : null}
                      {detailNote ? <span className="sdk-muted" style={{ fontSize: 13 }}>{detailNote}</span> : null}
                      <div style={{ fontSize: 14, whiteSpace: "pre-wrap" }}>
                        {openStory.summary ? (
                          <>
                            <Badge>{openStory.summaryOrigin === "inference" ? "AI 生成" : "人工"}</Badge>{" "}
                            {openStory.summary}
                          </>
                        ) : (
                          <span className="sdk-muted">暂无摘要。</span>
                        )}
                      </div>
                      <div className="sdk-muted" style={{ fontSize: 12 }}>
                        参与人物 {(openStory.participantIds ?? []).length} 人 · 创建于 {formatDateTime(openStory.createdAt)}
                      </div>
                      <div>
                        <button
                          type="button"
                          className="sdk-button sdk-button-primary"
                          disabled={busyStoryId === storyId}
                          onClick={() => void summarize(storyId)}
                        >
                          {busyStoryId === storyId ? "生成中…" : openStory.summary ? "重新生成 AI 摘要" : "生成 AI 摘要"}
                        </button>
                      </div>
                    </div>
                  ) : null}
                </li>
              );
            })}
          </ul>
        ) : null}
      </SectionCard>
    </div>
  );
}
