#!/usr/bin/env python3
"""Scaffolder for the missory PC people/memories/assistant capability packages."""
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
        "exports": {".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"}},
        "dependencies": deps,
    }, indent=2, ensure_ascii=False) + "\n"

REACT = {"react": "catalog:", "react-dom": "catalog:", "react-router-dom": "catalog:"}

# ---------------- pc-people ----------------
w("sdkwork-missory-pc-people/package.json", pkg_json("@sdkwork/missory-pc-people", {
    "@sdkwork/missory-pc-core": "workspace:*",
    "@sdkwork/missory-pc-commons": "workspace:*",
    **REACT,
}))

w("sdkwork-missory-pc-people/src/index.ts", '''export { PeopleListScreen } from "./screens/PeopleListScreen.tsx";
export { PersonDetailScreen } from "./screens/PersonDetailScreen.tsx";
''')

w("sdkwork-missory-pc-people/src/screens/PeopleListScreen.tsx", '''import { useCallback, useEffect, useState } from "react";
import { Link } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard, TextField,
  relationshipTypeLabel,
} from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryPerson } from "@sdkwork/missory-app-sdk";
import type { BootstrappedMissoryPcRuntime } from "../../../src/bootstrap/runtime";

export function PeopleListScreen({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
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
''')

w("sdkwork-missory-pc-people/src/screens/PersonDetailScreen.tsx", '''import { useCallback, useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard,
  formatDateTime, formatRelativeDays, importanceLabel, memoryTypeLabel,
  relationshipTypeLabel, truncate,
} from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type {
  MissoryPersonDetail, MissoryTimelineEntry,
} from "@sdkwork/missory-app-sdk";
import type { BootstrappedMissoryPcRuntime } from "../../../src/bootstrap/runtime";

export function PersonDetailScreen({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
  const { personId = "" } = useParams();
  const [detail, setDetail] = useState<MissoryPersonDetail | null>(null);
  const [timeline, setTimeline] = useState<MissoryTimelineEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [briefing, setBriefing] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [personDetail, timelinePage] = await Promise.all([
        runtime.people.retrieve(personId),
        runtime.people.timeline(personId),
      ]);
      setDetail(personDetail);
      setTimeline(timelinePage);
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setLoading(false);
    }
  }, [runtime, personId]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const buildBriefing = useCallback(async () => {
    const result = await runtime.assistant.briefing(personId);
    setBriefing(result.briefing);
  }, [runtime, personId]);

  if (loading) return <LoadingState />;
  if (error) return <ErrorState message={error} onRetry={() => void refresh()} />;
  if (!detail) return <EmptyState title="人物不存在" />;

  const stats = detail.stats;

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h2 style={{ margin: 0 }}>{detail.person.displayName}</h2>
        <Link to="/people" className="sdk-muted" style={{ fontSize: 13 }}>← 返回列表</Link>
      </div>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <SectionCard title="关于 TA">
          <div style={{ display: "grid", gap: 6, fontSize: 14 }}>
            {detail.person.title ? <div>职位：{detail.person.title}</div> : null}
            {detail.person.company ? <div>公司：{detail.person.company}</div> : null}
            {detail.person.city ? <div>城市：{detail.person.city}</div> : null}
            {detail.person.interests.length > 0 ? <div>兴趣：{detail.person.interests.join("、")}</div> : null}
            {detail.relationships ? (
              <div>
                关系：
                {detail.relationships.relationshipTypes.map((type) => (
                  <Badge key={type}>{relationshipTypeLabel(type)}</Badge>
                ))}
                {" "}
                <span className="sdk-muted">重要度 {importanceLabel(detail.relationships.importance ?? undefined)}</span>
              </div>
            ) : (
              <div className="sdk-muted">尚未建立关系档案</div>
            )}
          </div>
        </SectionCard>
        <SectionCard title="数据概览">
          <div style={{ display: "grid", gap: 6, fontSize: 14 }}>
            <div>认识天数：{stats.knownDays}</div>
            <div>
              最近联系：
              {stats.daysSinceLastContact === undefined || stats.daysSinceLastContact === null
                ? "暂无记录"
                : formatRelativeDays(stats.daysSinceLastContact)}
            </div>
            <div>记忆 {stats.memoryCount} 条 · 故事 {stats.storyCount} 个</div>
            <button type="button" className="sdk-button sdk-button-primary" onClick={() => void buildBriefing()}>
              生成见面简报
            </button>
            {briefing ? (
              <pre style={{ whiteSpace: "pre-wrap", fontSize: 13, margin: 0 }}>{briefing}</pre>
            ) : null}
          </div>
        </SectionCard>
      </div>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <SectionCard title="重要记忆">
          {detail.recentMemories.length === 0 ? (
            <EmptyState title="暂无记忆" />
          ) : (
            <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
              {detail.recentMemories.map((memory) => (
                <li key={String(memory.id)}>
                  <Badge>{memoryTypeLabel(memory.memoryType)}</Badge>{" "}
                  {truncate(memory.content, 46)}
                </li>
              ))}
            </ul>
          )}
        </SectionCard>
        <SectionCard title="你答应过 TA">
          {detail.commitments.length === 0 ? (
            <EmptyState title="没有待履行的承诺" />
          ) : (
            <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
              {detail.commitments.map((memory) => (
                <li key={String(memory.id)}>{truncate(memory.content, 46)}</li>
              ))}
            </ul>
          )}
        </SectionCard>
      </div>
      <SectionCard title="关系时间线">
        {timeline.length === 0 ? (
          <EmptyState title="时间线为空" />
        ) : (
          <ol style={{ margin: 0, paddingLeft: 20, display: "grid", gap: 6 }}>
            {timeline.map((entry, index) => (
              <li key={`${entry.occurredAt}-${index}`}>
                <span className="sdk-muted">{formatDateTime(entry.occurredAt)}</span>{" "}
                {entry.title}
              </li>
            ))}
          </ol>
        )}
      </SectionCard>
    </div>
  );
}
''')

w("sdkwork-missory-pc-people/README.md", '''# @sdkwork/missory-pc-people

People capability screens: list/search/create and the person profile with
relationships, memories, commitments, stats, timeline, and the AI briefing.
''')

# ---------------- pc-memories ----------------
w("sdkwork-missory-pc-memories/package.json", pkg_json("@sdkwork/missory-pc-memories", {
    "@sdkwork/missory-pc-core": "workspace:*",
    "@sdkwork/missory-pc-commons": "workspace:*",
    **REACT,
}))

w("sdkwork-missory-pc-memories/src/index.ts", '''export { MemoriesScreen } from "./screens/MemoriesScreen.tsx";
''')

w("sdkwork-missory-pc-memories/src/screens/MemoriesScreen.tsx", '''import { useCallback, useEffect, useState } from "react";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard, TextField,
  memoryTypeLabel, truncate,
} from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryMemory } from "@sdkwork/missory-app-sdk";
import type { BootstrappedMissoryPcRuntime } from "../../../src/bootstrap/runtime";

export function MemoriesScreen({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
  const [items, setItems] = useState<MissoryMemory[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [keyword, setKeyword] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const [extractText, setExtractText] = useState("");
  const [extractNote, setExtractNote] = useState<string | null>(null);

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

  const confirm = useCallback(async (memoryId: string) => {
    await runtime.memories.confirm(memoryId);
    await refresh();
  }, [runtime, refresh]);

  const reject = useCallback(async (memoryId: string) => {
    await runtime.memories.reject(memoryId);
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

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>记忆</h2>
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
                  <Badge>{memoryTypeLabel(memory.memoryType)}</Badge>{" "}
                  <Badge>{memory.origin === "inference" ? "推断" : "事实"}</Badge>{" "}
                  <Badge>{memory.status}</Badge>{" "}
                  {truncate(memory.content, 60)}
                  {memory.confidence !== undefined && memory.confidence !== null ? (
                    <span className="sdk-muted" style={{ fontSize: 12 }}> · 置信 {memory.confidence}</span>
                  ) : null}
                </span>
                {memory.status === "candidate" ? (
                  <span style={{ display: "flex", gap: 6 }}>
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
                  </span>
                ) : null}
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
''')

w("sdkwork-missory-pc-memories/README.md", '''# @sdkwork/missory-pc-memories

Memories capability screen: extraction, filtering, and the Fact ≠ Inference
confirm/reject workflow.
''')

# ---------------- pc-assistant ----------------
w("sdkwork-missory-pc-assistant/package.json", pkg_json("@sdkwork/missory-pc-assistant", {
    "@sdkwork/missory-pc-core": "workspace:*",
    "@sdkwork/missory-pc-commons": "workspace:*",
    **REACT,
}))

w("sdkwork-missory-pc-assistant/src/index.ts", '''export { AssistantScreen } from "./screens/AssistantScreen.tsx";
''')

w("sdkwork-missory-pc-assistant/src/screens/AssistantScreen.tsx", '''import { useCallback, useState } from "react";

import { EmptyState, SectionCard, TextField } from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryMessageDraft } from "@sdkwork/missory-app-sdk";
import type { BootstrappedMissoryPcRuntime } from "../../../src/bootstrap/runtime";

interface ChatTurn {
  role: "user" | "assistant";
  text: string;
}

export function AssistantScreen({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
  const [question, setQuestion] = useState("");
  const [turns, setTurns] = useState<ChatTurn[]>([]);
  const [busy, setBusy] = useState(false);
  const [personId, setPersonId] = useState("");
  const [draft, setDraft] = useState<MissoryMessageDraft | null>(null);

  const ask = useCallback(async () => {
    const text = question.trim();
    if (!text || busy) return;
    setBusy(true);
    setTurns((previous) => [...previous, { role: "user", text }]);
    setQuestion("");
    try {
      const answer = await runtime.assistant.query(text);
      setTurns((previous) => [...previous, { role: "assistant", text: answer.answer }]);
    } catch (cause) {
      setTurns((previous) => [
        ...previous,
        { role: "assistant", text: `出错了：${normalizeClientError(cause).message}` },
      ]);
    } finally {
      setBusy(false);
    }
  }, [runtime, question, busy]);

  const makeDraft = useCallback(async () => {
    if (!personId.trim()) return;
    setBusy(true);
    try {
      const result = await runtime.assistant.messageDraft(personId.trim(), "birthday", "warm");
      setDraft(result);
    } catch (cause) {
      setDraft(null);
      window.alert(normalizeClientError(cause).message);
    } finally {
      setBusy(false);
    }
  }, [runtime, personId]);

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>AI 社交助手</h2>
      <SectionCard title="问我任何关于你人际关系的问题">
        <div style={{ display: "grid", gap: 10 }}>
          <div
            style={{
              minHeight: 160, maxHeight: 320, overflow: "auto", display: "grid", gap: 8,
              border: "1px solid var(--sdk-color-border)", borderRadius: 10, padding: 12,
            }}
          >
            {turns.length === 0 ? (
              <EmptyState
                title="试试：李明是谁？"
                hint="也可以问：我和李明是什么关系？我多久没联系王强了？谁喜欢摄影？"
              />
            ) : (
              turns.map((turn, index) => (
                <div
                  key={index}
                  style={{
                    alignSelf: turn.role === "user" ? "flex-end" : "flex-start",
                    background: turn.role === "user" ? "var(--sdk-color-surface-muted)" : "transparent",
                    padding: turn.role === "user" ? "6px 10px" : 0,
                    borderRadius: 8,
                    maxWidth: "80%",
                    whiteSpace: "pre-wrap",
                  }}
                >
                  {turn.text}
                </div>
              ))
            )}
          </div>
          <div style={{ display: "flex", gap: 8 }}>
            <input
              className="sdk-input"
              value={question}
              placeholder="输入问题…"
              onChange={(event) => setQuestion(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") void ask();
              }}
            />
            <button type="button" className="sdk-button sdk-button-primary" disabled={busy} onClick={() => void ask()}>
              发送
            </button>
          </div>
        </div>
      </SectionCard>
      <SectionCard title="消息草稿（仅草稿，永不自动发送）">
        <div style={{ display: "flex", gap: 8, alignItems: "end" }}>
          <TextField label="人物 ID" value={personId} onChange={setPersonId} placeholder="人物详情页可见" />
          <button type="button" className="sdk-button" disabled={busy} onClick={() => void makeDraft()}>
            生成生日祝福草稿
          </button>
        </div>
        {draft ? (
          <div style={{ display: "grid", gap: 6 }}>
            <div
              style={{
                border: "1px dashed var(--sdk-color-border)", borderRadius: 10, padding: 12,
                whiteSpace: "pre-wrap",
              }}
            >
              {draft.draft}
            </div>
            <span className="sdk-muted" style={{ fontSize: 12 }}>{draft.disclaimer}</span>
          </div>
        ) : null}
      </SectionCard>
    </div>
  );
}
''')

w("sdkwork-missory-pc-assistant/README.md", '''# @sdkwork/missory-pc-assistant

Assistant capability screen: social Q&A chat and draft-only message drafting.
''')

print("people + memories + assistant written")
