import type { FieldEditEvent, InputEvent, MissoryAppInstance, TapEvent } from "../../typings/runtime";

const { page, resolveErrorMessage } = getApp<MissoryAppInstance>().runtime;

function splitList(raw: string): string[] {
  return raw
    .split(/[,，]/)
    .map((item) => item.trim())
    .filter(Boolean);
}

type PeopleForm = { displayName: string; city: string; tags: string };

type PeopleData = {
  persons: Array<{ id: string; name: string; title: string }>;
  keyword: string;
  loading: boolean;
  form: PeopleForm;
  creating: boolean;
  note: string;
};

type PeopleCustom = {
  refresh(): Promise<void>;
  onKeyword(event: InputEvent): void;
  onSearch(): void;
  onFormField(event: FieldEditEvent<keyof PeopleForm>): void;
  create(): Promise<void>;
};

Page<PeopleData, PeopleCustom>({
  data: {
    persons: [],
    keyword: "",
    loading: true,
    form: { displayName: "", city: "", tags: "" },
    creating: false,
    note: "",
  },
  onShow() {
    void this.refresh();
  },
  async refresh() {
    this.setData({ loading: true });
    const result = await page.client.missory.persons.list({ pageSize: 50, q: this.data.keyword || undefined });
    this.setData({
      loading: false,
      persons: result.items.map((person) => ({ id: String(person.id), name: person.displayName, title: person.title ?? "" })),
    });
  },
  onKeyword(event) {
    this.setData({ keyword: event.detail.value });
  },
  onSearch() {
    void this.refresh();
  },
  onFormField(event) {
    const field = event.currentTarget.dataset.field;
    this.setData({ form: { ...this.data.form, [field]: event.detail.value } });
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
      await this.refresh();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "创建失败") });
    } finally {
      this.setData({ creating: false });
    }
  },
});
