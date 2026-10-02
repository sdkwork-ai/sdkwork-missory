import { useCallback, useEffect, useState } from "react";

import { ErrorState, LoadingState, SectionCard, TextField } from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryPcRuntime } from "@sdkwork/missory-pc-core";

function splitList(value: string): string[] {
  return value.split(/[,，、]/u).map((item) => item.trim()).filter(Boolean);
}

/** 触发浏览器下载：data export JSON 文档（隐私数据导出，PRD §9）。 */
function downloadDataExport(payload: unknown) {
  const blob = new Blob([JSON.stringify(payload, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `missory-export-${Date.now()}.json`;
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  URL.revokeObjectURL(url);
}

export function ProfileScreen({ runtime }: { runtime: MissoryPcRuntime }) {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [note, setNote] = useState<string | null>(null);
  const [exporting, setExporting] = useState(false);
  const [exportNote, setExportNote] = useState<string | null>(null);

  const [displayName, setDisplayName] = useState("");
  const [nickname, setNickname] = useState("");
  const [city, setCity] = useState("");
  const [occupation, setOccupation] = useState("");
  const [company, setCompany] = useState("");
  const [education, setEducation] = useState("");
  const [interests, setInterests] = useState("");
  const [likes, setLikes] = useState("");
  const [dislikes, setDislikes] = useState("");
  const [communicationStyle, setCommunicationStyle] = useState("");
  const [bio, setBio] = useState("");

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const profile = await runtime.profile.retrieve();
      setDisplayName(profile.displayName);
      setNickname(profile.nickname ?? "");
      setCity(profile.city ?? "");
      setOccupation(profile.occupation ?? "");
      setCompany(profile.company ?? "");
      setEducation(profile.education ?? "");
      setInterests((profile.interests ?? []).join(","));
      setLikes((profile.likes ?? []).join(","));
      setDislikes((profile.dislikes ?? []).join(","));
      setCommunicationStyle(profile.communicationStyle ?? "");
      setBio(profile.bio ?? "");
    } catch (cause) {
      setError(normalizeClientError(cause).message);
    } finally {
      setLoading(false);
    }
  }, [runtime]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const save = useCallback(async () => {
    if (!displayName.trim() || saving) return;
    setSaving(true);
    setNote(null);
    try {
      await runtime.profile.update({
        displayName: displayName.trim(),
        nickname: nickname.trim() || undefined,
        city: city.trim() || undefined,
        occupation: occupation.trim() || undefined,
        company: company.trim() || undefined,
        education: education.trim() || undefined,
        interests: splitList(interests),
        likes: splitList(likes),
        dislikes: splitList(dislikes),
        communicationStyle: communicationStyle.trim() || undefined,
        bio: bio.trim() || undefined,
      });
      setNote("资料已保存。");
      await refresh();
    } catch (cause) {
      setNote(normalizeClientError(cause).message);
    } finally {
      setSaving(false);
    }
  }, [runtime, displayName, nickname, city, occupation, company, education, interests, likes, dislikes, communicationStyle, bio, saving, refresh]);

  const exportData = useCallback(async () => {
    if (exporting) return;
    setExporting(true);
    setExportNote(null);
    try {
      const data = await runtime.dataExports.exportData();
      downloadDataExport(data);
      setExportNote(`已导出 ${data.memories.length} 条记忆、${data.persons.length} 位人物。`);
    } catch (cause) {
      setExportNote(normalizeClientError(cause).message);
    } finally {
      setExporting(false);
    }
  }, [runtime, exporting]);

  if (loading) return <LoadingState />;
  if (error) return <ErrorState message={error} onRetry={() => void refresh()} />;

  return (
    <div style={{ display: "grid", gap: 16 }}>
      <h2 style={{ margin: 0 }}>我的资料</h2>
      <SectionCard title="基本信息">
        <div style={{ display: "grid", gap: 10 }}>
          <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
            <TextField label="姓名（必填）" value={displayName} onChange={setDisplayName} />
            <TextField label="昵称" value={nickname} onChange={setNickname} />
            <TextField label="城市" value={city} onChange={setCity} />
          </div>
          <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
            <TextField label="职业" value={occupation} onChange={setOccupation} />
            <TextField label="公司" value={company} onChange={setCompany} />
            <TextField label="教育经历" value={education} onChange={setEducation} />
          </div>
        </div>
      </SectionCard>
      <SectionCard title="画像与偏好（帮助 AI 更懂你）">
        <div style={{ display: "grid", gap: 10 }}>
          <TextField label="兴趣（多个用逗号分隔）" value={interests} onChange={setInterests} placeholder="例如：摄影, 马拉松" />
          <TextField label="喜欢（多个用逗号分隔）" value={likes} onChange={setLikes} placeholder="例如：爬山, 咖啡" />
          <TextField label="不喜欢（多个用逗号分隔）" value={dislikes} onChange={setDislikes} placeholder="例如：香菜" />
          <TextField label="沟通风格" value={communicationStyle} onChange={setCommunicationStyle} placeholder="例如：直接、简洁" />
          <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
            <span className="sdk-muted">个人简介</span>
            <textarea
              className="sdk-input"
              rows={3}
              value={bio}
              onChange={(event) => setBio(event.target.value)}
            />
          </label>
          <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
            <button
              type="button"
              className="sdk-button sdk-button-primary"
              disabled={saving || !displayName.trim()}
              onClick={() => void save()}
            >
              保存资料
            </button>
            {note ? <span className="sdk-muted" style={{ fontSize: 13 }}>{note}</span> : null}
          </div>
        </div>
      </SectionCard>
      <SectionCard title="隐私与数据">
        <div style={{ display: "grid", gap: 8 }}>
          <span className="sdk-muted" style={{ fontSize: 13 }}>
            导出包含资料、人物、关系、记忆、故事与提醒的完整 JSON 文档，仅在本地下载，不会发送给任何第三方。
          </span>
          <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
            <button
              type="button"
              className="sdk-button"
              disabled={exporting}
              onClick={() => void exportData()}
            >
              {exporting ? "导出中…" : "导出我的数据"}
            </button>
            {exportNote ? <span className="sdk-muted" style={{ fontSize: 13 }}>{exportNote}</span> : null}
          </div>
        </div>
      </SectionCard>
    </div>
  );
}
