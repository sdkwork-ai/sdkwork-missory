const { page } = getApp().runtime;
Page({
  data: { reminders: [], memories: [], persons: [], loading: true, error: "" },
  onShow() { this.refresh(); },
  async refresh() {
    this.setData({ loading: true, error: "" });
    try {
      const digest = await page.client.missory.home.today.list();
      this.setData({
        loading: false,
        reminders: digest.todayReminders.map((r) => ({ reminderId: r.reminderId, name: r.personName ?? "", title: r.title, type: r.type })),
        memories: digest.recentMemories.map((m) => ({ id: String(m.id), content: m.content })),
        persons: digest.recentPersons.map((p) => ({ id: String(p.id), name: p.displayName })),
      });
    } catch (e) {
      this.setData({ loading: false, error: (e && e.message) || "加载失败" });
    }
  },
  async dismiss(event) {
    await page.client.missory.reminders.dismiss(event.currentTarget.dataset.id);
    this.refresh();
  },
});
