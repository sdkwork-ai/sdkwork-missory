import { useCallback, useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import {
  Badge, EmptyState, ErrorState, LoadingState, SectionCard,
  formatDateTime, formatRelativeDays, importanceLabel, memoryTypeLabel,
  relationshipTypeLabel, truncate,
} from "@sdkwork/missory-h5-commons";
import { normalizeClientError } from "@sdkwork/missory-h5-core";

import type {
  MissoryPersonDetail, MissoryTimelineEntry,
} from "@sdkwork/missory-app-sdk";
import type { MissoryH5Runtime } from "@sdkwork/missory-h5-core";

export function PersonDetailScreen({ runtime }: { runtime: MissoryH5Runtime }) {
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
