const { page } = getApp().runtime;

const SNOOZE_DAYS = [1, 3, 7];

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
  async snooze(event) {
    let choice;
    try {
      choice = await wx.showActionSheet({ itemList: SNOOZE_DAYS.map((days) => "推迟 " + days + " 天") });
    } catch {
      return;
    }
    await page.client.missory.reminders.snooze(event.currentTarget.dataset.id, { days: SNOOZE_DAYS[choice.tapIndex] });
    this.refresh();
  },
  logout() {
    page.session.logout();
    wx.reLaunch({ url: "/pages/login/index" });
  },
});
