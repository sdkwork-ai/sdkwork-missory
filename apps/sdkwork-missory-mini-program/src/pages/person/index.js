const { page } = getApp().runtime;
Page({
  data: { person: null, memories: [], timeline: [], briefing: "" },
  onLoad(query) {
    this.personId = query.id;
    this.refresh();
  },
  async refresh() {
    const detail = await page.client.missory.persons.retrieve(this.personId);
    const timeline = await page.client.missory.persons.timeline.list(this.personId);
    this.setData({
      person: {
        name: detail.person.displayName,
        title: detail.person.title ?? "",
        city: detail.person.city ?? "",
        knownDays: detail.stats.knownDays,
        memoryCount: detail.stats.memoryCount,
      },
      memories: detail.recentMemories.map((m) => ({ id: String(m.id), content: m.content, type: m.type })),
      timeline: timeline.items.map((t) => ({ at: (t.occurredAt || "").slice(0, 10), title: t.title })),
    });
  },
  async brief() {
    const result = await page.client.missoryAssistant.assistant.briefings.create({ personId: this.personId });
    this.setData({ briefing: result.briefing });
  },
});
