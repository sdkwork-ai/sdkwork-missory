const { page } = getApp().runtime;

function splitList(raw) {
  return String(raw || "")
    .split(/[,，]/)
    .map((item) => item.trim())
    .filter(Boolean);
}

Page({
  data: {
    persons: [],
    keyword: "",
    loading: true,
    form: { displayName: "", city: "", tags: "" },
    creating: false,
    note: "",
  },
  onShow() { this.refresh(); },
  async refresh() {
    this.setData({ loading: true });
    const result = await page.client.missory.persons.list({ pageSize: 50, q: this.data.keyword || undefined });
    this.setData({ loading: false, persons: result.items.map((p) => ({ id: String(p.id), name: p.displayName, title: p.title ?? "" })) });
  },
  onKeyword(event) { this.setData({ keyword: event.detail.value }); },
  onSearch() { this.refresh(); },
  onFormField(event) {
    this.setData({ ["form." + event.currentTarget.dataset.field]: event.detail.value });
  },
  async create() {
    const displayName = this.data.form.displayName.trim();
    if (!displayName || this.data.creating) return;
    this.setData({ creating: true, note: "" });
    try {
      const tags = splitList(this.data.form.tags);
      await page.client.missory.persons.create({
        displayName,
        city: this.data.form.city.trim() || undefined,
        ...(tags.length > 0 ? { tags } : {}),
      });
      this.setData({ form: { displayName: "", city: "", tags: "" }, note: "已创建人物" });
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "创建失败" });
    } finally {
      this.setData({ creating: false });
    }
  },
});
