export function EmptyState({ title, hint }: { title: string; hint?: string }) {
  return (
    <div style={{ padding: 20, textAlign: "center", display: "grid", gap: 6 }}>
      <div style={{ fontWeight: 600 }}>{title}</div>
      {hint ? <div className="sdk-muted" style={{ fontSize: 13 }}>{hint}</div> : null}
    </div>
  );
}
