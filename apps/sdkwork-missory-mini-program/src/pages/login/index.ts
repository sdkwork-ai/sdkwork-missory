// Credential-entry login (IAM app-api): POST /app/v3/api/auth/sessions with the
// deployment-provisioned bootstrap Access-Token; success stores the dual-token
// pair and relaunches into the console.
import type { InputEvent, MissoryAppInstance } from "../../typings/runtime";

const { page, resolveErrorMessage } = getApp<MissoryAppInstance>().runtime;

type LoginData = { account: string; password: string; error: string; pending: boolean };

type LoginCustom = {
  onAccount(event: InputEvent): void;
  onPassword(event: InputEvent): void;
  submit(): Promise<void>;
};

Page<LoginData, LoginCustom>({
  data: { account: "", password: "", error: "", pending: false },
  onAccount(event) {
    this.setData({ account: event.detail.value });
  },
  onPassword(event) {
    this.setData({ password: event.detail.value });
  },
  async submit() {
    const { account, password, pending } = this.data;
    if (pending) return;
    this.setData({ error: "", pending: true });
    try {
      await page.session.loginWithPassword({ account, password });
      wx.reLaunch({ url: "/pages/home/index" });
    } catch (error) {
      this.setData({ error: resolveErrorMessage(error, "登录失败") });
    } finally {
      this.setData({ pending: false });
    }
  },
});
