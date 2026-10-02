// IAM credential-entry session for the mini-program (mirrors pc-core/h5-core).
//
// `POST /app/v3/api/auth/sessions` is a credential-entry route: it requires
// only the deployment-provisioned bootstrap `Access-Token` and returns the
// dual-token pair every app-api request then carries through the shared token
// manager. Tokens persist in WeChat storage so a cold start stays signed in.
// Development without a provisioned bootstrap token rides the gateway's IAM
// dev authentication fallback (any value authenticates); non-development
// deployments inject the bootstrap token through the runtime-env build step.

import { createTokenManager, type AuthTokenManager } from "@sdkwork/sdk-common";

import type { MissoryMpEnvironment } from "./runtime.ts";

const STORAGE_KEY = "sdkwork.missory.mp.session";

export const SESSION_EXPIRED_EVENT = "sdkwork:session-expired";

export interface MissoryMpStoredSession {
  authToken: string;
  accessToken: string;
  refreshToken?: string;
  displayName?: string;
}

export class IamLoginError extends Error {
  readonly code: number | undefined;
  readonly status: number;

  constructor(message: string, options: { code?: number; status: number }) {
    super(message);
    this.name = "IamLoginError";
    this.code = options.code;
    this.status = options.status;
  }
}

export interface MissoryMpSessionFacade {
  readonly tokenManager: AuthTokenManager;
  isAuthenticated(): boolean;
  loginWithPassword(input: { account: string; password: string }): Promise<MissoryMpStoredSession>;
  logout(): void;
}

function loadStoredSession(): MissoryMpStoredSession | undefined {
  try {
    const raw = wx.getStorageSync(STORAGE_KEY);
    if (typeof raw !== "string" || raw.length === 0) return undefined;
    const parsed: unknown = JSON.parse(raw);
    if (!parsed || typeof parsed !== "object") return undefined;
    const session = parsed as Record<string, unknown>;
    if (typeof session.authToken !== "string" || typeof session.accessToken !== "string") {
      return undefined;
    }
    return parsed as MissoryMpStoredSession;
  } catch {
    return undefined;
  }
}

function saveStoredSession(session: MissoryMpStoredSession): void {
  wx.setStorageSync(STORAGE_KEY, JSON.stringify(session));
}

function clearStoredSession(): void {
  try {
    wx.removeStorageSync(STORAGE_KEY);
  } catch {
    // storage removal is best-effort; the in-memory pair is already cleared
  }
}

function bootstrapAccessToken(environment: MissoryMpEnvironment): string {
  if (environment.authBootstrapAccessToken) return environment.authBootstrapAccessToken;
  if (environment.environment === "development") return "dev-bootstrap-access-token";
  throw new IamLoginError("缺少部署预置的凭证入口 Access-Token（runtime-env authBootstrapAccessToken）", {
    status: 0,
  });
}

function problemMessage(status: number, payload: unknown): IamLoginError {
  if (payload && typeof payload === "object") {
    const problem = payload as Record<string, unknown>;
    const detail = typeof problem.detail === "string" ? problem.detail : undefined;
    const code = typeof problem.code === "number" ? problem.code : undefined;
    if (detail) return new IamLoginError(detail, { code, status });
  }
  return new IamLoginError(`登录失败（HTTP ${status}）`, { status });
}

export function createMissoryMpTokenManager(environment: MissoryMpEnvironment): AuthTokenManager {
  const tokenManager = createTokenManager();
  const stored = loadStoredSession();
  if (stored) {
    tokenManager.setTokens({
      authToken: stored.authToken,
      accessToken: stored.accessToken,
      refreshToken: stored.refreshToken,
    });
    return tokenManager;
  }
  if (environment.environment === "development") {
    tokenManager.setAuthToken("dev-auth-token");
    tokenManager.setAccessToken("dev-access-token");
  }
  return tokenManager;
}

export function createMissoryMpSessionFacade(
  environment: MissoryMpEnvironment,
  tokenManager: AuthTokenManager,
): MissoryMpSessionFacade {
  const sessionsUrl = `${environment.appApiBaseUrl.replace(/\/+$/, "")}/app/v3/api/auth/sessions`;

  const applySession = (session: MissoryMpStoredSession): MissoryMpStoredSession => {
    tokenManager.setTokens({
      authToken: session.authToken,
      accessToken: session.accessToken,
      refreshToken: session.refreshToken,
    });
    saveStoredSession(session);
    return session;
  };

  return {
    tokenManager,
    isAuthenticated: () => tokenManager.hasAuthToken() && tokenManager.hasAccessToken(),
    async loginWithPassword({ account, password }) {
      const trimmed = account.trim();
      if (!trimmed || !password) {
        throw new IamLoginError("请输入账号和密码", { status: 0 });
      }
      let response: Response;
      try {
        response = await fetch(sessionsUrl, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "Access-Token": bootstrapAccessToken(environment),
          },
          body: JSON.stringify({ grantType: "password", username: trimmed, password }),
        });
      } catch (error) {
        throw new IamLoginError(
          `无法连接登录服务：${error instanceof Error ? error.message : String(error)}`,
          { status: 0 },
        );
      }
      if (!response.ok) {
        let payload: unknown = undefined;
        try {
          payload = JSON.parse(await response.text());
        } catch {
          payload = undefined;
        }
        throw problemMessage(response.status, payload);
      }
      let body: unknown;
      try {
        body = JSON.parse(await response.text());
      } catch {
        throw new IamLoginError("登录响应格式无效", { status: 200 });
      }
      const envelope = (body ?? {}) as Record<string, unknown>;
      if (envelope.code !== 0 || !envelope.data || typeof envelope.data !== "object") {
        throw new IamLoginError("登录响应状态异常", { status: 200 });
      }
      const data = envelope.data as Record<string, unknown>;
      const authToken = typeof data.authToken === "string" ? data.authToken : undefined;
      const accessToken = typeof data.accessToken === "string" ? data.accessToken : undefined;
      if (!authToken || !accessToken) {
        throw new IamLoginError("登录响应缺少双令牌", { status: 200 });
      }
      const user = (data.user ?? {}) as Record<string, unknown>;
      return applySession({
        authToken,
        accessToken,
        refreshToken: typeof data.refreshToken === "string" ? data.refreshToken : undefined,
        displayName: typeof user.displayName === "string" ? user.displayName : undefined,
      });
    },
    logout() {
      tokenManager.clearTokens();
      clearStoredSession();
    },
  };
}
