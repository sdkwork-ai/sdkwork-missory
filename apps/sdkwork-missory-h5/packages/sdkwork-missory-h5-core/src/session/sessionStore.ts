// Persisted dual-token session for the H5 console (localStorage).
//
// The IAM login flow stores the session pair after
// `POST /app/v3/api/auth/sessions` succeeds; `createTokenManagerFor` restores
// it on boot so a refresh keeps the user signed in. Development without a
// stored session keeps the well-known dev identity (gateway DEV_AUTH_BYPASS).

const STORAGE_KEY = "sdkwork.missory.h5.session";

export interface MissoryStoredSession {
  authToken: string;
  accessToken: string;
  refreshToken?: string;
  displayName?: string;
  username?: string;
}

function isSession(value: unknown): value is MissoryStoredSession {
  if (!value || typeof value !== "object") return false;
  const session = value as Record<string, unknown>;
  return typeof session.authToken === "string" && typeof session.accessToken === "string";
}

export function loadStoredSession(storage: Storage | undefined = globalThis.localStorage): MissoryStoredSession | undefined {
  try {
    const raw = storage?.getItem(STORAGE_KEY);
    if (!raw) return undefined;
    const parsed: unknown = JSON.parse(raw);
    return isSession(parsed) ? parsed : undefined;
  } catch {
    return undefined;
  }
}

export function saveStoredSession(session: MissoryStoredSession, storage: Storage | undefined = globalThis.localStorage): void {
  storage?.setItem(STORAGE_KEY, JSON.stringify(session));
}

export function clearStoredSession(storage: Storage | undefined = globalThis.localStorage): void {
  storage?.removeItem(STORAGE_KEY);
}
