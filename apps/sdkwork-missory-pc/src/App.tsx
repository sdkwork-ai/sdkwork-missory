import { useCallback, useEffect, useState, type ReactNode } from "react";
import { Navigate, Route, Routes } from "react-router-dom";
import { BrowserRouter } from "react-router-dom";

import { SESSION_EXPIRED_EVENT } from "@sdkwork/missory-pc-core";
import { AssistantScreen } from "@sdkwork/missory-pc-assistant";
import { MemoriesScreen } from "@sdkwork/missory-pc-memories";
import { PeopleListScreen, PersonDetailScreen } from "@sdkwork/missory-pc-people";
import { HomeScreen, LoginScreen, MissoryAppShell } from "@sdkwork/missory-pc-shell";

import type { BootstrappedMissoryPcRuntime } from "./bootstrap/runtime";

/**
 * Installs the session-expiry boundary once per page load: any app-api
 * response answered 401/403 dispatches SESSION_EXPIRED_EVENT so the session
 * gate re-renders to the login screen. The SDK client itself stays untouched
 * (generated output is generator-owned).
 */
function useSessionExpiryBoundary(onExpired: () => void) {
  useEffect(() => {
    const handle = () => onExpired();
    window.addEventListener(SESSION_EXPIRED_EVENT, handle);
    return () => window.removeEventListener(SESSION_EXPIRED_EVENT, handle);
  }, [onExpired]);
}

/**
 * Session gate: development with the gateway bypass runs signed-in by default
 * (dev identity seeded in the token manager); every other environment renders
 * the credential-entry login until a dual-token session exists. A mid-session
 * 401 clears the stored session and returns the user to login.
 */
function SessionGate({
  runtime,
  children,
}: {
  runtime: BootstrappedMissoryPcRuntime;
  children: ReactNode;
}) {
  const bypassed = runtime.config.environment === "development";
  const [authenticated, setAuthenticated] = useState(
    bypassed || runtime.session.isAuthenticated(),
  );
  const expire = useCallback(() => {
    runtime.session.logout();
    setAuthenticated(false);
  }, [runtime]);
  useSessionExpiryBoundary(expire);

  if (!authenticated) {
    return <LoginScreen runtime={runtime} onSessionEstablished={() => setAuthenticated(true)} />;
  }
  return <>{children}</>;
}

export function App({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
  const handleLogout = useCallback(() => {
    window.location.assign("/");
    window.location.reload();
  }, []);
  return (
    <BrowserRouter>
      <SessionGate runtime={runtime}>
        <Routes>
          <Route element={<MissoryAppShell runtime={runtime} onSessionEnded={handleLogout} />}>
            <Route index element={<HomeScreen runtime={runtime} />} />
            <Route path="people" element={<PeopleListScreen runtime={runtime} />} />
            <Route path="people/:personId" element={<PersonDetailScreen runtime={runtime} />} />
            <Route path="memories" element={<MemoriesScreen runtime={runtime} />} />
            <Route path="assistant" element={<AssistantScreen runtime={runtime} />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Route>
        </Routes>
      </SessionGate>
    </BrowserRouter>
  );
}
