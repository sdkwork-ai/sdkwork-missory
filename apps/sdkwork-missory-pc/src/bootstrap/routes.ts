// Client route table (single source for the shell navigation and lazy routes).
export interface MissoryRoute {
  path: string;
  label: string;
}

export const clientRoutes: MissoryRoute[] = [
  { path: "/", label: "首页" },
  { path: "/people", label: "人物" },
  { path: "/memories", label: "记忆" },
  { path: "/stories", label: "故事" },
  { path: "/assistant", label: "AI" },
  { path: "/profile", label: "我的" },
];
