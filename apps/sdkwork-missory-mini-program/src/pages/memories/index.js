const { page } = getApp().runtime;
Page({
  data: { memories: [], text: "", note: "", loading: true },
  onShow() { this.refresh(); },
  async refresh() {
    const result = await page.client.missory.memories.list({ pageSize: 50 });
    this.setData({ loading: false, memories: result.items.map((m) => ({ id: String(m.id), content: m.content, type: m.type, origin: m.origin, status: m.status })) });
  },
  onText(event) { this.setData({ text: event.detail.value }); },
  async extract() {
    if (!this.data.text.trim()) return;
    const result = await page.client.missory.memories.extract({ text: this.data.text });
    this.setData({ text: "", note: "已提取 " + result.items.length + " 条候选记忆" });
    this.refresh();
  },
  async confirm(event) { await page.client.missory.memories.confirm(event.currentTarget.dataset.id); this.refresh(); },
  async reject(event) { await page.client.missory.memories.reject(event.currentTarget.dataset.id); this.refresh(); },
});
