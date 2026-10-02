const { page } = getApp().runtime;

Page({
  data: {
    stories: [],
    loading: true,
    note: "",
    form: { title: "", location: "", startedAt: "", endedAt: "" },
    creating: false,
  },
  onShow() { this.refresh(); },
  async refresh() {
    this.setData({ loading: true });
    const result = await page.client.missory.stories.list({ pageSize: 50 });
    this.setData({
      loading: false,
      stories: result.items.map((s) => ({
        id: String(s.id),
        title: s.title,
        summary: s.summary ?? "",
        meta: [s.location ?? "", [(s.startedAt || "").slice(0, 10), (s.endedAt || "").slice(0, 10)].filter(Boolean).join(" ~ ")]
          .filter(Boolean)
          .join(" · "),
      })),
    });
  },
  onFormField(event) {
    this.setData({ ["form." + event.currentTarget.dataset.field]: event.detail.value });
  },
  onStartDate(event) { this.setData({ "form.startedAt": event.detail.value }); },
  onEndDate(event) { this.setData({ "form.endedAt": event.detail.value }); },
  async create() {
    const title = this.data.form.title.trim();
    if (!title || this.data.creating) return;
    this.setData({ creating: true, note: "" });
    try {
      await page.client.missory.stories.create({
        title,
        location: this.data.form.location.trim() || undefined,
        startedAt: this.data.form.startedAt || undefined,
        endedAt: this.data.form.endedAt || undefined,
      });
      this.setData({ form: { title: "", location: "", startedAt: "", endedAt: "" }, note: "已创建故事" });
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "创建失败" });
    } finally {
      this.setData({ creating: false });
    }
  },
  async summarize(event) {
    this.setData({ note: "正在生成 AI 摘要…" });
    try {
      await page.client.missoryAssistant.stories.summaries.create(event.currentTarget.dataset.id);
      this.setData({ note: "" });
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "摘要生成失败" });
    }
  },
});
