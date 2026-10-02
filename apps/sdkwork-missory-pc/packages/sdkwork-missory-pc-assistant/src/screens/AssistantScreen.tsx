import { useCallback, useState } from "react";

import { EmptyState, SectionCard, TextField } from "@sdkwork/missory-pc-commons";
import { normalizeClientError } from "@sdkwork/missory-pc-core";

import type { MissoryMessageDraft } from "@sdkwork/missory-app-sdk";
import type { MissoryPcRuntime } from "@sdkwork/missory-pc-core";

interface ChatTurn {
  role: "user" | "assistant";
  text: string;
}

export function AssistantScreen({ runtime }: { runtime: MissoryPcRuntime }) {
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
