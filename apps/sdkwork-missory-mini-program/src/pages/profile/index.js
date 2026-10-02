const { page } = getApp().runtime;

function splitList(raw) {
  return String(raw || "")
    .split(/[,，]/)
    .map((item) => item.trim())
    .filter(Boolean);
}

function joinList(raw) {
  return (raw ?? []).join("，");
}

Page({
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
  onLoad() { this.refresh(); },
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
    } catch (e) {
      this.setData({ loading: false, note: (e && e.message) || "加载失败" });
    }
  },
  onFormField(event) {
    this.setData({ ["form." + event.currentTarget.dataset.field]: event.detail.value });
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
    } catch (e) {
      this.setData({ note: (e && e.message) || "保存失败" });
    } finally {
      this.setData({ saving: false });
    }
  },
  async exportData() {
    if (this.data.saving) return;
    this.setData({ saving: true, note: "" });
    try {
      const item = await page.client.missory.dataExports.create({});
      const payload = item && item.exportedAt ? item : (item && item.item) || item;
      wx.setClipboardData({
        data: JSON.stringify(payload ?? {}, null, 2),
        success: () => {
          this.setData({ note: "导出成功，完整 JSON 已复制到剪贴板" });
        },
      });
    } catch (e) {
      this.setData({ note: (e && e.message) || "导出失败" });
    } finally {
      this.setData({ saving: false });
    }
  },
});
