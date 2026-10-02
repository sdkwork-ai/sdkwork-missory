export function ErrorState({ message, onRetry }: { message: string; onRetry?: () => void }) {
  return (
    <div
      role="alert"
      style={{
        padding: 16,
        border: "1px solid var(--sdk-color-danger)",
        borderRadius: 10,
        display: "grid",
        gap: 8,
      }}
    >
      <div style={{ color: "var(--sdk-color-danger)" }}>{message}</div>
      {onRetry ? (
        <button type="button" className="sdk-button" onClick={onRetry}>
          重试
        </button>
      ) : null}
    </div>
  );
}
