import type {
  CheckboxGroupChangeEvent,
  FieldEditEvent,
  MissoryAppInstance,
  TapEvent,
} from "../../typings/runtime";

const { page, resolveErrorMessage } = getApp<MissoryAppInstance>().runtime;

const RELATIONSHIP_TYPE_LABELS = {
  family: "家人",
  friend: "朋友",
  classmate: "同学",
  colleague: "同事",
  client: "客户",
  partner: "合作伙伴",
  teacher: "老师",
  student: "学生",
  neighbor: "邻居",
  spouse: "配偶",
  other: "其他",
} as const;

type RelationshipType = keyof typeof RELATIONSHIP_TYPE_LABELS;

const RELATIONSHIP_TYPE_OPTIONS: Array<{ value: RelationshipType; label: string }> = Object.entries(
  RELATIONSHIP_TYPE_LABELS,
).map(([value, label]) => ({ value: value as RelationshipType, label }));

function isRelationshipType(value: string): value is RelationshipType {
  return value in RELATIONSHIP_TYPE_LABELS;
}

type PersonDetail = Awaited<ReturnType<typeof page.client.missory.persons.retrieve>>;

type PersonEditForm = { displayName: string; title: string; city: string };

type PersonData = {
  person: { name: string; title: string; city: string; knownDays: number; memoryCount: number } | null;
  relationships: Array<{ id: string; label: string }>;
  memories: Array<{ id: string; content: string; type: string }>;
  timeline: Array<{ at: string; title: string }>;
  briefing: string;
  editing: boolean;
  editForm: PersonEditForm;
  relationshipOptions: Array<{ value: RelationshipType; label: string; checked?: boolean }>;
  selectedRelationshipTypes: string[];
  savingRelationship: boolean;
  note: string;
};

type PersonCustom = {
  personId: string;
  personDetail: PersonDetail | null;
  refresh(): Promise<void>;
  brief(): Promise<void>;
  startEdit(): void;
  cancelEdit(): void;
  onEditField(event: FieldEditEvent<keyof PersonEditForm>): void;
  saveEdit(): Promise<void>;
  remove(): Promise<void>;
  onRelationshipChange(event: CheckboxGroupChangeEvent): void;
  saveRelationship(): Promise<void>;
  removeRelationship(event: TapEvent<{ id: string }>): Promise<void>;
};

Page<PersonData, PersonCustom>({
  personId: "",
  personDetail: null,
  data: {
    person: null,
    relationships: [],
    memories: [],
    timeline: [],
    briefing: "",
    editing: false,
    editForm: { displayName: "", title: "", city: "" },
    relationshipOptions: RELATIONSHIP_TYPE_OPTIONS,
    selectedRelationshipTypes: [],
    savingRelationship: false,
    note: "",
  },
  onLoad(query) {
    this.personId = query.id ?? "";
    this.personDetail = null;
    void this.refresh();
  },
  async refresh() {
    const detail = await page.client.missory.persons.retrieve(this.personId);
    this.personDetail = detail;
    const timeline = await page.client.missory.persons.timeline.list(this.personId);
    const selected = new Set<RelationshipType>();
    for (const edge of detail.relationships ?? []) {
      for (const relationshipType of edge.relationshipTypes ?? []) {
        if (isRelationshipType(relationshipType)) selected.add(relationshipType);
      }
    }
    this.setData({
      person: {
        name: detail.person.displayName,
        title: detail.person.title ?? "",
        city: detail.person.city ?? "",
        knownDays: detail.stats.knownDays,
        memoryCount: detail.stats.memoryCount,
      },
      relationships: (detail.relationships ?? []).map((edge) => ({
        id: String(edge.id),
        label: (edge.relationshipTypes ?? [])
          .map((relationshipType) => RELATIONSHIP_TYPE_LABELS[relationshipType] ?? relationshipType)
          .join(" / "),
      })),
      relationshipOptions: RELATIONSHIP_TYPE_OPTIONS.map((option) => ({
        ...option,
        checked: selected.has(option.value),
      })),
      selectedRelationshipTypes: [...selected],
      memories: detail.recentMemories.map((memory) => ({
        id: String(memory.id),
        content: memory.content,
        type: memory.type,
      })),
      timeline: timeline.items.map((entry) => ({
        at: (entry.occurredAt || "").slice(0, 10),
        title: entry.title,
      })),
    });
  },
  async brief() {
    const result = await page.client.missoryAssistant.assistant.briefings.create({ personId: this.personId });
    this.setData({ briefing: result.briefing });
  },
  startEdit() {
    const person = this.personDetail?.person;
    if (!person) return;
    this.setData({
      editing: true,
      note: "",
      editForm: { displayName: person.displayName ?? "", title: person.title ?? "", city: person.city ?? "" },
    });
  },
  cancelEdit() {
    this.setData({ editing: false });
  },
  onEditField(event) {
    const field = event.currentTarget.dataset.field;
    this.setData({ editForm: { ...this.data.editForm, [field]: event.detail.value } });
  },
  async saveEdit() {
    const displayName = this.data.editForm.displayName.trim();
    const detail = this.personDetail;
    if (!displayName || !detail) return;
    const person = detail.person;
    try {
      // PUT carries the untouched profile fields through so the update keeps them.
      await page.client.missory.persons.update(this.personId, {
        displayName,
        title: this.data.editForm.title.trim() || undefined,
        city: this.data.editForm.city.trim() || undefined,
        aliases: person.aliases,
        gender: person.gender,
        birthday: person.birthday,
        company: person.company,
        avatarUrl: person.avatarUrl,
        tags: person.tags,
        interests: person.interests,
        preferences: person.preferences,
        bio: person.bio,
        contactChannels: person.contactChannels,
        notes: person.notes,
      });
      this.setData({ editing: false, note: "" });
      await this.refresh();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "保存失败") });
    }
  },
  async remove() {
    const confirmation = await wx.showModal({
      title: "删除人物",
      content: "将同时删除该人物的关系与记忆关联，确定删除？",
      confirmText: "删除",
      confirmColor: "#dc2626",
    });
    if (!confirmation.confirm) return;
    try {
      await page.client.missory.persons.delete(this.personId);
      wx.navigateBack();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "删除失败") });
    }
  },
  onRelationshipChange(event) {
    this.setData({ selectedRelationshipTypes: event.detail.value });
  },
  async saveRelationship() {
    if (this.data.savingRelationship) return;
    if (this.data.selectedRelationshipTypes.length === 0) {
      this.setData({ note: "请至少选择一种关系" });
      return;
    }
    this.setData({ savingRelationship: true, note: "" });
    try {
      await page.client.missory.relationships.create(this.personId, {
        relationshipTypes: this.data.selectedRelationshipTypes.filter(isRelationshipType),
      });
      await this.refresh();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "保存失败") });
    } finally {
      this.setData({ savingRelationship: false });
    }
  },
  async removeRelationship(event) {
    const confirmation = await wx.showModal({
      title: "解除关系",
      content: "确定解除与该人物的关系记录？",
      confirmText: "解除",
    });
    if (!confirmation.confirm) return;
    try {
      await page.client.missory.relationships.delete(this.personId, event.currentTarget.dataset.id);
      await this.refresh();
    } catch (error) {
      this.setData({ note: resolveErrorMessage(error, "解除失败") });
    }
  },
});
