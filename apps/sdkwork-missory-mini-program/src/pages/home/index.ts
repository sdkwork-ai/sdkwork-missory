import type { MissoryAppInstance, TapEvent } from "../../typings/runtime";

const { page, resolveErrorMessage } = getApp<MissoryAppInstance>().runtime;

const SNOOZE_DAYS = [1, 3, 7] as const;

type HomeData = {
  reminders: Array<{ reminderId: string; name: string; title: string; type: string }>;
  memories: Array<{ id: string; content: string }>;
  persons: Array<{ id: string; name: string }>;
  loading: boolean;
  error: string;
};

type HomeCustom = {
  refresh(): Promise<void>;
  dismiss(event: TapEvent<{ id: string }>): Promise<void>;
  snooze(event: TapEvent<{ id: string }>): Promise<void>;
  logout(): void;
};

Page<HomeData, HomeCustom>({
  data: { reminders: [], memories: [], persons: [], loading: true, error: "" },
  onShow() {
    void this.refresh();
  },
  async refresh() {
    this.setData({ loading: true, error: "" });
    try {
      const digest = await page.client.missory.home.today.list();
      this.setData({
        loading: false,
        reminders: digest.todayReminders.map((reminder) => ({
          reminderId: String(reminder.reminderId),
          name: reminder.personName ?? "",
          title: reminder.title,
          type: reminder.type,
        })),
        memories: digest.recentMemories.map((memory) => ({ id: String(memory.id), content: memory.content })),
        persons: digest.recentPersons.map((person) => ({ id: String(person.id), name: person.displayName })),
      });
    } catch (error) {
      this.setData({ loading: false, error: resolveErrorMessage(error, "加载失败") });
    }
  },
  async dismiss(event) {
    await page.client.missory.reminders.dismiss(event.currentTarget.dataset.id);
    await this.refresh();
  },
  async snooze(event) {
    const choice = await wx
      .showActionSheet({ itemList: SNOOZE_DAYS.map((days) => `推迟 ${days} 天`) })
      .catch(() => null);
    if (!choice) return;
    const days = SNOOZE_DAYS[choice.tapIndex];
    if (days === undefined) return;
    await page.client.missory.reminders.snooze(event.currentTarget.dataset.id, { days });
    await this.refresh();
  },
  logout() {
    page.session.logout();
    wx.reLaunch({ url: "/pages/login/index" });
  },
});
