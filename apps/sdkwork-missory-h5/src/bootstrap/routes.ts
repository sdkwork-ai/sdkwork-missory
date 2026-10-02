// H5 route table (stack/tab presentation per APP_MOBILE_REACT_UI_SPEC).
export interface MissoryH5Route {
  path: string;
  label: string;
  presentation: "stack" | "tab";
}

export const clientRoutes: MissoryH5Route[] = [
  { path: "/", label: "首页", presentation: "tab" },
  { path: "/people", label: "人物", presentation: "tab" },
  { path: "/memories", label: "记忆", presentation: "tab" },
  { path: "/assistant", label: "AI", presentation: "tab" },
  { path: "/people/:personId", label: "人物详情", presentation: "stack" },
];
