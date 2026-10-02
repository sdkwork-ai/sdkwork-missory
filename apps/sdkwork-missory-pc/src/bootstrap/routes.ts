// Client route table (single source for the shell navigation and lazy routes).
export interface MissoryRoute {
  path: string;
  label: string;
}

export const clientRoutes: MissoryRoute[] = [
  { path: "/", label: "首页" },
  { path: "/people", label: "人物" },
  { path: "/memories", label: "记忆" },
  { path: "/assistant", label: "AI" },
];
