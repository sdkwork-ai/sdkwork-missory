import { useCallback, useEffect, useState, type ReactNode } from "react";
import { Navigate, Route, Routes } from "react-router-dom";
import { HashRouter } from "react-router-dom";

import { SESSION_EXPIRED_EVENT } from "@sdkwork/missory-h5-core";
import {
  AssistantScreen,
  HomeScreen,
  LoginScreen,
  MemoriesScreen,
  AppErrorBoundary,
  MissoryAppShell,
  PeopleListScreen,
  PersonDetailScreen,
  ProfileScreen,
  StoriesScreen,
} from "@sdkwork/missory-h5-shell";

import type { BootstrappedMissoryH5Runtime } from "./bootstrap/runtime";

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
 * the credential-entry login until a dual-token session exists.
 */
function SessionGate({
  runtime,
  children,
}: {
  runtime: BootstrappedMissoryH5Runtime;
  children: ReactNode;
}) {
  const [authenticated, setAuthenticated] = useState(runtime.session.isAuthenticated());
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

export function App({ runtime }: { runtime: BootstrappedMissoryH5Runtime }) {
  const handleLogout = useCallback(() => {
    window.location.reload();
  }, []);
  return (
    <HashRouter>
      <SessionGate runtime={runtime}>
        <AppErrorBoundary>
        <Routes>
          <Route element={<MissoryAppShell runtime={runtime} onSessionEnded={handleLogout} />}>
            <Route index element={<HomeScreen runtime={runtime} />} />
            <Route path="people" element={<PeopleListScreen runtime={runtime} />} />
            <Route path="people/:personId" element={<PersonDetailScreen runtime={runtime} />} />
            <Route path="memories" element={<MemoriesScreen runtime={runtime} />} />
            <Route path="stories" element={<StoriesScreen runtime={runtime} />} />
            <Route path="assistant" element={<AssistantScreen runtime={runtime} />} />
            <Route path="profile" element={<ProfileScreen runtime={runtime} />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Route>
        </Routes>
        </AppErrorBoundary>
      </SessionGate>
    </HashRouter>
  );
}
