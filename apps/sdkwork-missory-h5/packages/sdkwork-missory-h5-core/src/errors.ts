/** Normalized client failure carrying the problem-detail code when present. */
export class MissoryClientError extends Error {
  readonly status: number;
  readonly code: number | null;
  readonly traceId: string | null;

  constructor(message: string, status: number, code: number | null, traceId: string | null) {
    super(message);
    this.name = "MissoryClientError";
    this.status = status;
    this.code = code;
    this.traceId = traceId;
  }
}

export function normalizeClientError(error: unknown): MissoryClientError {
  if (error instanceof MissoryClientError) return error;
  const payload = error as { status?: unknown; code?: unknown; message?: unknown; traceId?: unknown; body?: unknown };
  const status = typeof payload?.status === "number" ? payload.status : 0;
  const code = typeof payload?.code === "number" ? payload.code : null;
  const traceId = typeof payload?.traceId === "string" ? payload.traceId : null;
  const detail =
    (payload?.body as { detail?: unknown })?.detail ??
    (typeof payload?.message === "string" ? payload.message : undefined);
  return new MissoryClientError(
    typeof detail === "string" ? detail : "请求失败，请稍后重试",
    status,
    code,
    traceId,
  );
}
