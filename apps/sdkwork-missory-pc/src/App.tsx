import { Navigate, Route, Routes } from "react-router-dom";
import { BrowserRouter } from "react-router-dom";

import { AssistantScreen } from "@sdkwork/missory-pc-assistant";
import { MemoriesScreen } from "@sdkwork/missory-pc-memories";
import { PeopleListScreen, PersonDetailScreen } from "@sdkwork/missory-pc-people";
import { HomeScreen, MissoryAppShell } from "@sdkwork/missory-pc-shell";

import type { BootstrappedMissoryPcRuntime } from "./bootstrap/runtime";

export function App({ runtime }: { runtime: BootstrappedMissoryPcRuntime }) {
  return (
    <BrowserRouter>
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
    </BrowserRouter>
  );
}
