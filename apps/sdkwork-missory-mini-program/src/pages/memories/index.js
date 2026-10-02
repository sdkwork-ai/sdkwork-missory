const { page } = getApp().runtime;

const MEMORY_TYPE_OPTIONS = [
  { value: "semantic", label: "事实" },
  { value: "episodic", label: "事件" },
  { value: "temporal", label: "时间" },
  { value: "relationship", label: "关系" },
  { value: "preference", label: "偏好" },
  { value: "commitment", label: "承诺" },
];

Page({
  data: {
    memories: [],
    text: "",
    note: "",
    loading: true,
    persons: [],
    personIndex: 0,
    memoryTypeOptions: MEMORY_TYPE_OPTIONS,
    typeIndex: 0,
    content: "",
    creating: false,
  },
  onShow() { this.refresh(); },
  async refresh() {
    const [memoryResult, personResult] = await Promise.all([
      page.client.missory.memories.list({ pageSize: 50 }),
      page.client.missory.persons.list({ pageSize: 200 }),
    ]);
    this.setData({
      loading: false,
      memories: memoryResult.items.map((m) => ({ id: String(m.id), content: m.content, type: m.type, origin: m.origin, status: m.status })),
      persons: personResult.items.map((p) => ({ id: String(p.id), name: p.displayName })),
    });
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
  onPersonChange(event) { this.setData({ personIndex: Number(event.detail.value) }); },
  onTypeChange(event) { this.setData({ typeIndex: Number(event.detail.value) }); },
  onContent(event) { this.setData({ content: event.detail.value }); },
  async createMemory() {
    if (this.data.creating) return;
    const person = this.data.persons[this.data.personIndex];
    const content = this.data.content.trim();
    if (!person) {
      this.setData({ note: "请先在人物页创建人物" });
      return;
    }
    if (!content) {
      this.setData({ note: "请填写记忆内容" });
      return;
    }
    this.setData({ creating: true, note: "" });
    try {
      await page.client.missory.memories.create({
        personId: person.id,
        type: MEMORY_TYPE_OPTIONS[this.data.typeIndex].value,
        content,
      });
      this.setData({ content: "", note: "已记录记忆" });
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "记录失败" });
    } finally {
      this.setData({ creating: false });
    }
  },
  async remove(event) {
    const confirmation = await wx.showModal({
      title: "删除记忆",
      content: "确定删除这条记忆？",
      confirmText: "删除",
      confirmColor: "#dc2626",
    });
    if (!confirmation.confirm) return;
    try {
      await page.client.missory.memories.delete(event.currentTarget.dataset.id);
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "删除失败" });
    }
  },
});
