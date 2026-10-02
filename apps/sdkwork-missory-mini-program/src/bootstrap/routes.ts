// Route ids for the P0 pages (single source mirrored by src/app.json).
export const mpRoutes = [
  { id: "missory.home", pagePath: "pages/home/index", title: "首页" },
  { id: "missory.people", pagePath: "pages/people/index", title: "人物" },
  { id: "missory.person", pagePath: "pages/person/index", title: "人物详情" },
  { id: "missory.memories", pagePath: "pages/memories/index", title: "记忆" },
  { id: "missory.assistant", pagePath: "pages/assistant/index", title: "AI" },
  { id: "missory.stories", pagePath: "pages/stories/index", title: "故事" },
  { id: "missory.profile", pagePath: "pages/profile/index", title: "我的资料" },
] as const;
