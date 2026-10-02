const { page } = getApp().runtime;
Page({
  data: { persons: [], keyword: "", loading: true },
  onShow() { this.refresh(); },
  async refresh() {
    this.setData({ loading: true });
    const result = await page.client.missory.persons.list({ pageSize: 50, q: this.data.keyword || undefined });
    this.setData({ loading: false, persons: result.items.map((p) => ({ id: String(p.id), name: p.displayName, title: p.title ?? "" })) });
  },
  onKeyword(event) { this.setData({ keyword: event.detail.value }); },
  onSearch() { this.refresh(); },
});
