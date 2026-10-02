export interface NavigationModule {
  id: string;
  path: string;
  label: string;
}

export const navigationModules: NavigationModule[] = [
  { id: "home", path: "/", label: "首页" },
  { id: "people", path: "/people", label: "人物" },
  { id: "memories", path: "/memories", label: "记忆" },
  { id: "assistant", path: "/assistant", label: "AI" },
];
