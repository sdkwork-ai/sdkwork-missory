import { useState, type FormEvent } from "react";

import { IamLoginError, type MissoryH5Runtime } from "@sdkwork/missory-h5-core";

// Credential-entry login screen: account (username / email / phone) +
// password against `POST /app/v3/api/auth/sessions`. On success the session
// pair lands in the shared token manager + localStorage and the caller gates
// the console through.
export function LoginScreen({
  runtime,
  onSessionEstablished,
}: {
  runtime: MissoryH5Runtime;
  onSessionEstablished: () => void;
}) {
  const [account, setAccount] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | undefined>(undefined);
  const [pending, setPending] = useState(false);

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    setPending(true);
    setError(undefined);
    try {
      await runtime.session.loginWithPassword({ account, password });
      onSessionEstablished();
    } catch (cause) {
      setError(
        cause instanceof IamLoginError
          ? cause.message
          : cause instanceof Error
            ? cause.message
            : String(cause),
      );
    } finally {
      setPending(false);
    }
  };

  return (
    <div
      style={{
        minHeight: "100vh",
        display: "grid",
        placeItems: "center",
        background: "var(--sdk-color-surface)",
      }}
    >
      <form
        onSubmit={submit}
        style={{
          width: 360,
          display: "grid",
          gap: 12,
          padding: 28,
          border: "1px solid var(--sdk-color-border)",
          borderRadius: 12,
        }}
      >
        <div style={{ fontWeight: 700, fontSize: 20 }}>念忆 · Missory 登录</div>
        <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
          账号（用户名 / 邮箱 / 手机号）
          <input
            value={account}
            onChange={(event) => setAccount(event.target.value)}
            autoComplete="username"
            required
            style={{
              padding: "8px 10px",
              borderRadius: 8,
              border: "1px solid var(--sdk-color-border)",
            }}
          />
        </label>
        <label style={{ display: "grid", gap: 4, fontSize: 13 }}>
          密码
          <input
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            autoComplete="current-password"
            required
            style={{
              padding: "8px 10px",
              borderRadius: 8,
              border: "1px solid var(--sdk-color-border)",
            }}
          />
        </label>
        {error ? (
          <div role="alert" style={{ color: "var(--sdk-color-danger)", fontSize: 13 }}>
            {error}
          </div>
        ) : null}
        <button
          type="submit"
          disabled={pending}
          style={{
            padding: "10px 12px",
            borderRadius: 8,
            border: "none",
            background: "var(--sdk-color-primary)",
            color: "var(--sdk-color-primary-contrast)",
            fontWeight: 600,
            cursor: pending ? "wait" : "pointer",
          }}
        >
          {pending ? "登录中…" : "登录"}
        </button>
        <div className="sdk-muted" style={{ fontSize: 11 }}>
          {runtime.config.profileId} · {runtime.config.environment}
        </div>
      </form>
    </div>
  );
}
