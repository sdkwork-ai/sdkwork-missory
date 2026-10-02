import { Navigate, Route, Routes } from "react-router-dom";
import { HashRouter } from "react-router-dom";

import {
  AssistantScreen,
  HomeScreen,
  MemoriesScreen,
  MissoryAppShell,
  PeopleListScreen,
  PersonDetailScreen,
} from "@sdkwork/missory-h5-shell";

import type { BootstrappedMissoryH5Runtime } from "./bootstrap/runtime";

export function App({ runtime }: { runtime: BootstrappedMissoryH5Runtime }) {
  return (
    <HashRouter>
      <Routes>
        <Route element={<MissoryAppShell runtime={runtime} />}>
          <Route index element={<HomeScreen runtime={runtime} />} />
          <Route path="people" element={<PeopleListScreen runtime={runtime} />} />
          <Route path="people/:personId" element={<PersonDetailScreen runtime={runtime} />} />
          <Route path="memories" element={<MemoriesScreen runtime={runtime} />} />
          <Route path="assistant" element={<AssistantScreen runtime={runtime} />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}
