import type { DateChangeEvent, FieldEditEvent, MissoryAppInstance, TapEvent } from "../../typings/runtime";

const { page, resolveErrorMessage } = getApp<MissoryAppInstance>().runtime;

type StoriesForm = { title: string; location: string; startedAt: string; endedAt: string };

type StoriesData = {
  stories: Array<{ id: string; title: string; summary: string; meta: string }>;
  loading: boolean;
  note: string;
  form: StoriesForm;
  creating: boolean;
};

type StoriesCustom = {
  refresh(): Promise<void>;
  onFormField(event: FieldEditEvent<keyof StoriesForm>): void;
  onStartDate(event: DateChangeEvent): void;
  onEndDate(event: DateChangeEvent): void;
  create(): Promise<void>;
  summarize(event: TapEvent<{ id: string }>): Promise<void>;
};

Page<StoriesData, StoriesCustom>({
  data: {
    stories: [],
    loading: true,
    note: "",
    form: { title: "", location: "", startedAt: "", endedAt: "" },
    creating: false,
  },
  onShow() {
    void this.refresh();
  },
  async refresh() {
    this.setData({ loading: true });
    const result = await page.client.missory.stories.list({ pageSize: 50 });
    this.setData({
      loading: false,
      stories: result.items.map((story) => ({
        id: String(story.id),
        title: story.title,
        summary: story.summary ?? "",
        meta: [story.location ?? "", [(story.startedAt || "").slice(0, 10), (story.endedAt || "").slice(0, 10)].filter(Boolean).join(" ~ ")]
          .filter(Boolean)
          .join(" · "),
      })),
    });
  },
  onFormField(event) {
    const field = event.currentTarget.dataset.field;
    this.setData({ form: { ...this.data.form, [field]: event.detail.value } });
  },
  onStartDate(event) {
    this.setData({ form: { ...this.data.form, startedAt: event.detail.value } });
  },
  onEndDate(event) {
    this.setData({ form: { ...this.data.form, endedAt: event.detail.value } });
  },
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
      await this.refresh();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "创建失败") });
    } finally {
      this.setData({ creating: false });
    }
  },
  async summarize(event) {
    this.setData({ note: "正在生成 AI 摘要…" });
    try {
      await page.client.missoryAssistant.stories.summaries.create(event.currentTarget.dataset.id);
      this.setData({ note: "" });
      await this.refresh();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "摘要生成失败") });
    }
  },
});
