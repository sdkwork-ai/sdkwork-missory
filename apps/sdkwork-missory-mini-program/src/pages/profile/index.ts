import type { FieldEditEvent, MissoryAppInstance } from "../../typings/runtime";

const { page, resolveErrorMessage } = getApp<MissoryAppInstance>().runtime;

function splitList(raw: string): string[] {
  return raw
    .split(/[,，]/)
    .map((item) => item.trim())
    .filter(Boolean);
}

function joinList(raw: ReadonlyArray<string> | undefined): string {
  return (raw ?? []).join("，");
}

type ProfileForm = {
  displayName: string;
  nickname: string;
  city: string;
  occupation: string;
  company: string;
  education: string;
  interests: string;
  likes: string;
  dislikes: string;
  communicationStyle: string;
  bio: string;
};

type ProfileData = {
  form: ProfileForm;
  loading: boolean;
  saving: boolean;
  note: string;
};

type ProfileCustom = {
  refresh(): Promise<void>;
  onFormField(event: FieldEditEvent<keyof ProfileForm>): void;
  save(): Promise<void>;
  exportData(): Promise<void>;
};

Page<ProfileData, ProfileCustom>({
  data: {
    form: {
      displayName: "",
      nickname: "",
      city: "",
      occupation: "",
      company: "",
      education: "",
      interests: "",
      likes: "",
      dislikes: "",
      communicationStyle: "",
      bio: "",
    },
    loading: true,
    saving: false,
    note: "",
  },
  onLoad() {
    void this.refresh();
  },
  async refresh() {
    this.setData({ loading: true, note: "" });
    try {
      const profile = await page.client.missory.myProfile.retrieve();
      this.setData({
        loading: false,
        form: {
          displayName: profile.displayName ?? "",
          nickname: profile.nickname ?? "",
          city: profile.city ?? "",
          occupation: profile.occupation ?? "",
          company: profile.company ?? "",
          education: profile.education ?? "",
          interests: joinList(profile.interests),
          likes: joinList(profile.likes),
          dislikes: joinList(profile.dislikes),
          communicationStyle: profile.communicationStyle ?? "",
          bio: profile.bio ?? "",
        },
      });
    } catch (error) {
      this.setData({ loading: false, note: resolveErrorMessage(error, "加载失败") });
    }
  },
  onFormField(event) {
    const field = event.currentTarget.dataset.field;
    this.setData({ form: { ...this.data.form, [field]: event.detail.value } });
  },
  async save() {
    if (this.data.saving) return;
    const displayName = this.data.form.displayName.trim();
    if (!displayName) {
      this.setData({ note: "请填写姓名" });
      return;
    }
    this.setData({ saving: true, note: "" });
    try {
      await page.client.missory.myProfile.update({
        displayName,
        nickname: this.data.form.nickname.trim(),
        city: this.data.form.city.trim(),
        occupation: this.data.form.occupation.trim(),
        company: this.data.form.company.trim(),
        education: this.data.form.education.trim(),
        interests: splitList(this.data.form.interests),
        likes: splitList(this.data.form.likes),
        dislikes: splitList(this.data.form.dislikes),
        communicationStyle: this.data.form.communicationStyle.trim(),
        bio: this.data.form.bio.trim(),
      });
      this.setData({ note: "已保存资料" });
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "保存失败") });
    } finally {
      this.setData({ saving: false });
    }
  },
  async exportData() {
    if (this.data.saving) return;
    this.setData({ saving: true, note: "" });
    try {
      const item: unknown = await page.client.missory.dataExports.create({});
      // Whole-account privacy export: accept both the bare item and a
      // `{ item }` envelope from older deployments before copying the JSON.
      const record = (item ?? {}) as Record<string, unknown>;
      const payload = "exportedAt" in record && record.exportedAt ? record : (record.item ?? record);
      wx.setClipboardData({
        data: JSON.stringify(payload ?? {}, null, 2),
        success: () => {
          this.setData({ note: "导出成功，完整 JSON 已复制到剪贴板" });
        },
      });
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "导出失败") });
    } finally {
      this.setData({ saving: false });
    }
  },
});
