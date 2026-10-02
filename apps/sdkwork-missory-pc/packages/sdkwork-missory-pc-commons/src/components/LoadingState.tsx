export function LoadingState({ label = "加载中…" }: { label?: string }) {
  return (
    <div className="sdk-muted" style={{ padding: 16, textAlign: "center" }} role="status">
      {label}
    </div>
  );
}
