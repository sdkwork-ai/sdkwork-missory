// IAM credential-entry login for the PC console (TECH_ARCHITECTURE §8 item 2).
//
// `POST /app/v3/api/auth/sessions` is a credential-entry route: it requires
// only the deployment-provisioned bootstrap `Access-Token` (tenant isolation;
// `Authorization` must be absent). The response carries the dual-token pair
// the app-api surface consumes on every subsequent request:
// `Authorization: Bearer <authToken>` + `Access-Token: <accessToken>`.
//
// Development note: the standalone gateway accepts any bootstrap value while
// the IAM dev authentication fallback is active; non-development deployments
// provision the bootstrap token through the browser runtime-env document.

import type { AuthTokenManager } from "@sdkwork/sdk-common";

import type { MissoryPcRuntimeConfig } from "../config/runtime-config.ts";
import {
  clearStoredSession,
  loadStoredSession,
  saveStoredSession,
  type MissoryStoredSession,
} from "./sessionStore.ts";

export const SESSION_EXPIRED_EVENT = "sdkwork:session-expired";

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

export interface MissorySessionFacade {
  readonly tokenManager: AuthTokenManager;
  isAuthenticated(): boolean;
  loginWithPassword(input: { account: string; password: string }): Promise<MissoryStoredSession>;
  logout(): void;
}

function bootstrapAccessToken(config: MissoryPcRuntimeConfig): string {
  if (config.authBootstrapAccessToken) return config.authBootstrapAccessToken;
  if (config.environment === "development") return "dev-bootstrap-access-token";
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

function sessionFromResponse(body: unknown): MissoryStoredSession {
  if (!body || typeof body !== "object") {
    throw new IamLoginError("登录响应格式无效", { status: 200 });
  }
  const envelope = body as Record<string, unknown>;
  if (envelope.code !== 0 || !envelope.data || typeof envelope.data !== "object") {
    const message = typeof envelope.message === "string" ? envelope.message : "登录响应状态异常";
    throw new IamLoginError(message, { status: 200 });
  }
  const data = envelope.data as Record<string, unknown>;
  const authToken = typeof data.authToken === "string" ? data.authToken : undefined;
  const accessToken = typeof data.accessToken === "string" ? data.accessToken : undefined;
  if (!authToken || !accessToken) {
    throw new IamLoginError("登录响应缺少双令牌", { status: 200 });
  }
  const user = (data.user ?? {}) as Record<string, unknown>;
  return {
    authToken,
    accessToken,
    refreshToken: typeof data.refreshToken === "string" ? data.refreshToken : undefined,
    displayName: typeof user.displayName === "string" ? user.displayName : undefined,
  };
}

export function createMissorySessionFacade(
  config: MissoryPcRuntimeConfig,
  tokenManager: AuthTokenManager,
): MissorySessionFacade {
  const sessionsUrl = `${config.appApiBaseUrl.replace(/\/+$/, "")}/app/v3/api/auth/sessions`;

  const applySession = (session: MissoryStoredSession): MissoryStoredSession => {
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
            "Access-Token": bootstrapAccessToken(config),
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
          payload = await response.json();
        } catch {
          payload = undefined;
        }
        throw problemMessage(response.status, payload);
      }
      return applySession(sessionFromResponse(await response.json()));
    },
    logout() {
      tokenManager.clearTokens();
      clearStoredSession();
    },
  };
}

export { loadStoredSession, saveStoredSession, clearStoredSession };
export type { MissoryStoredSession };
