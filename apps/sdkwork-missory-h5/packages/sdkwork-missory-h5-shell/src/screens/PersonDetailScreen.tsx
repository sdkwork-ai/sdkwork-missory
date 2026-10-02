import { useCallback, useEffect, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard, TextField,
  formatDateTime, formatRelativeDays, importanceLabel, memoryTypeLabel,
  relationshipTypeLabel, truncate,
} from "@sdkwork/missory-h5-commons";
import { normalizeClientError } from "@sdkwork/missory-h5-core";

import type {
  MissoryPerson, MissoryPersonDetail, MissoryRelationshipType,
  MissoryTimelineEntry,
} from "@sdkwork/missory-app-sdk";
import type { MissoryH5Runtime } from "@sdkwork/missory-h5-core";

const RELATIONSHIP_TYPES: MissoryRelationshipType[] = [
  "family", "friend", "classmate", "colleague", "client", "partner",
  "teacher", "student", "neighbor", "spouse", "other",
];

function splitList(value: string): string[] {
  return value.split(/[,，、]/u).map((item) => item.trim()).filter(Boolean);
}

export function PersonDetailScreen({ runtime }: { runtime: MissoryH5Runtime }) {
  const { personId = "" } = useParams();
  const navigate = useNavigate();
  const [detail, setDetail] = useState<MissoryPersonDetail | null>(null);
  const [timeline, setTimeline] = useState<MissoryTimelineEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [briefing, setBriefing] = useState<string | null>(null);

  // 编辑人物
  const [editing, setEditing] = useState(false);
  const [saving, setSaving] = useState(false);
  const [editDisplayName, setEditDisplayName] = useState("");
  const [editTitle, setEditTitle] = useState("");
  const [editCompany, setEditCompany] = useState("");
  const [editCity, setEditCity] = useState("");
  const [editInterests, setEditInterests] = useState("");
  const [editBio, setEditBio] = useState("");

  // 关系档案维护
  const [persons, setPersons] = useState<MissoryPerson[]>([]);
  const [relPersonId, setRelPersonId] = useState("");
  const [relType, setRelType] = useState<MissoryRelationshipType>("friend");
  const [relBusy, setRelBusy] = useState(false);
  const [relNote, setRelNote] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [personDetail, timelinePage, peoplePage] = await Promise.all([
        runtime.people.retrieve(personId),
        runtime.people.timeline(personId),
        runtime.people.list({ pageSize: 200 }),
      ]);
      setDetail(personDetail);
      setTimeline(timelinePage);
      setPersons(peoplePage.items);
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

  const startEdit = useCallback(() => {
    if (!detail) return;
    const person = detail.person;
    setEditDisplayName(person.displayName);
    setEditTitle(person.title ?? "");
    setEditCompany(person.company ?? "");
    setEditCity(person.city ?? "");
    setEditInterests((person.interests ?? []).join(","));
    setEditBio(person.bio ?? "");
    setEditing(true);
  }, [detail]);

  const saveEdit = useCallback(async () => {
    if (!detail || !editDisplayName.trim() || saving) return;
    setSaving(true);
    setError(null);
    try {
      const person = detail.person;
      await runtime.people.update(personId, {
        aliases: person.aliases,
        gender: person.gender,
        birthday: person.birthday,
        avatarUrl: person.avatarUrl,
        tags: person.tags,
        preferences: person.preferences,
        contactChannels: person.contactChannels,
        notes: person.notes,
        displayName: editDisplayName.trim(),
        title: editTitle.trim() || undefined,
        company: editCompany.trim() || undefined,
        city: editCity.trim() || undefined,
        interests: splitList(editInterests),
        bio: editBio.trim() || undefined,
      });
      setEditing(false);
      await refresh();
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setSaving(false);
    }
  }, [runtime, detail, personId, editDisplayName, editTitle, editCompany, editCity, editInterests, editBio, saving, refresh]);

  const deletePerson = useCallback(async () => {
    if (!detail) return;
    const confirmed = window.confirm(
      `确定删除「${detail.person.displayName}」吗？TA 的记忆、故事与关系档案将一并删除，且不可恢复。`,
    );
    if (!confirmed) return;
    try {
      await runtime.people.delete(personId);
      navigate("/people");
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    }
  }, [runtime, detail, personId, navigate]);

  const addRelationship = useCallback(async () => {
    const targetPersonId = relPersonId || personId;
    if (!targetPersonId || relBusy) return;
    setRelBusy(true);
    setRelNote(null);
    try {
      await runtime.people.upsertRelationship(targetPersonId, {
        relationshipTypes: [relType],
      });
      setRelNote("关系已保存。");
      await refresh();
    } catch (cause) {
      setRelNote(normalizeClientError(cause).message);
    } finally {
      setRelBusy(false);
    }
  }, [runtime, relPersonId, personId, relType, relBusy, refresh]);

  const deleteRelationship = useCallback(async (relationshipId: string) => {
    if (!window.confirm("确定删除这条关系档案吗？")) return;
    setRelNote(null);
    try {
      await runtime.people.deleteRelationship(personId, relationshipId);
      await refresh();
    } catch (cause) {
      setRelNote(normalizeClientError(cause).message);
    }
  }, [runtime, personId, refresh]);

  if (loading) return <LoadingState />;
  if (error && !detail) return <ErrorState message={error} onRetry={() => void refresh()} />;
  if (!detail) return <EmptyState title="人物不存在" />;

  const stats = detail.stats;

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h2 style={{ margin: 0 }}>{detail.person.displayName}</h2>
        <div style={{ display: "flex", gap: 10, alignItems: "center" }}>
          <button type="button" className="sdk-button" onClick={startEdit}>
            编辑资料
          </button>
          <button type="button" className="sdk-button" onClick={() => void deletePerson()}>
            删除人物
          </button>
          <Link to="/people" className="sdk-muted" style={{ fontSize: 13 }}>← 返回列表</Link>
        </div>
      </div>
      {error ? <ErrorState message={error} onRetry={() => void refresh()} /> : null}
      {editing ? (
        <SectionCard title="编辑人物资料">
          <div style={{ display: "grid", gap: 10 }}>
            <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
              <TextField label="姓名（必填）" value={editDisplayName} onChange={setEditDisplayName} />
              <TextField label="职位" value={editTitle} onChange={setEditTitle} />
              <TextField label="公司" value={editCompany} onChange={setEditCompany} />
              <TextField label="城市" value={editCity} onChange={setEditCity} />
            </div>
            <TextField label="兴趣（多个用逗号分隔）" value={editInterests} onChange={setEditInterests} placeholder="例如：摄影, 马拉松" />
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">备注</span>
              <textarea
                className="sdk-input"
                rows={3}
                value={editBio}
                onChange={(event) => setEditBio(event.target.value)}
              />
            </label>
            <div style={{ display: "flex", gap: 8 }}>
              <button
                type="button"
                className="sdk-button sdk-button-primary"
                disabled={saving || !editDisplayName.trim()}
                onClick={() => void saveEdit()}
              >
                保存
              </button>
              <button type="button" className="sdk-button" onClick={() => setEditing(false)}>
                取消
              </button>
            </div>
          </div>
        </SectionCard>
      ) : null}
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <SectionCard title="关于 TA">
          <div style={{ display: "grid", gap: 6, fontSize: 14 }}>
            {detail.person.title ? <div>职位：{detail.person.title}</div> : null}
            {detail.person.company ? <div>公司：{detail.person.company}</div> : null}
            {detail.person.city ? <div>城市：{detail.person.city}</div> : null}
            {(detail.person.interests ?? []).length > 0 ? <div>兴趣：{(detail.person.interests ?? []).join("、")}</div> : null}
            {detail.relationships && detail.relationships.length > 0 ? (
              <div>
                关系：
                {detail.relationships[0].relationshipTypes.map((type) => (
                  <Badge key={type}>{relationshipTypeLabel(type)}</Badge>
                ))}
                {" "}
                <span className="sdk-muted">重要度 {importanceLabel(detail.relationships[0].importance ?? undefined)}</span>
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
      <SectionCard title="关系档案">
        <div style={{ display: "grid", gap: 8 }}>
          <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
            {(detail.relationships ?? []).length === 0 ? (
              <li className="sdk-muted">还没有关系档案，可在下方添加。</li>
            ) : (
              (detail.relationships ?? []).map((relationship) => (
                <li
                  key={String(relationship.id)}
                  style={{
                    display: "flex", justifyContent: "space-between", alignItems: "center", gap: 12,
                    borderBottom: "1px solid var(--sdk-color-border)", paddingBottom: 8,
                  }}
                >
                  <span>
                    {relationship.relationshipTypes.map((type) => (
                      <Badge key={type}>{relationshipTypeLabel(type)}</Badge>
                    ))}
                    {" "}
                    <span className="sdk-muted" style={{ fontSize: 12 }}>
                      重要度 {importanceLabel(relationship.importance ?? undefined)}
                      {relationship.startedAt ? ` · 始于 ${formatDateTime(relationship.startedAt)}` : ""}
                    </span>
                  </span>
                  <button
                    type="button"
                    className="sdk-button"
                    onClick={() => void deleteRelationship(String(relationship.id))}
                  >
                    删除
                  </button>
                </li>
              ))
            )}
          </ul>
          <div style={{ display: "flex", gap: 8, alignItems: "end", flexWrap: "wrap" }}>
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">关联人物</span>
              <select
                className="sdk-input"
                style={{ width: 160 }}
                value={relPersonId || personId}
                onChange={(event) => setRelPersonId(event.target.value)}
              >
                {(persons.length === 0 ? [] : persons).map((person) => (
                  <option key={String(person.id)} value={String(person.id)}>
                    {person.displayName}
                  </option>
                ))}
              </select>
            </label>
            <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
              <span className="sdk-muted">关系类型</span>
              <select
                className="sdk-input"
                style={{ width: 130 }}
                value={relType}
                onChange={(event) => setRelType(event.target.value as MissoryRelationshipType)}
              >
                {RELATIONSHIP_TYPES.map((type) => (
                  <option key={type} value={type}>{relationshipTypeLabel(type)}</option>
                ))}
              </select>
            </label>
            <button
              type="button"
              className="sdk-button sdk-button-primary"
              disabled={relBusy}
              onClick={() => void addRelationship()}
            >
              添加关系
            </button>
            {relNote ? <span className="sdk-muted" style={{ fontSize: 13 }}>{relNote}</span> : null}
          </div>
        </div>
      </SectionCard>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <SectionCard title="重要记忆">
          {detail.recentMemories.length === 0 ? (
            <EmptyState title="暂无记忆" />
          ) : (
            <ul style={{ margin: 0, padding: 0, listStyle: "none", display: "grid", gap: 8 }}>
              {detail.recentMemories.map((memory) => (
                <li key={String(memory.id)}>
                  <Badge>{memoryTypeLabel(memory.type)}</Badge>{" "}
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
