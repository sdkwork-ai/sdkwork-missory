const { page } = getApp().runtime;

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
};

const RELATIONSHIP_TYPE_OPTIONS = Object.keys(RELATIONSHIP_TYPE_LABELS).map((value) => ({ value, label: RELATIONSHIP_TYPE_LABELS[value] }));

Page({
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
    this.personId = query.id;
    this.personDetail = null;
    this.refresh();
  },
  async refresh() {
    const detail = await page.client.missory.persons.retrieve(this.personId);
    this.personDetail = detail;
    const timeline = await page.client.missory.persons.timeline.list(this.personId);
    const selected = new Set();
    for (const edge of detail.relationships ?? []) {
      for (const relationshipType of edge.relationshipTypes ?? []) selected.add(relationshipType);
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
        label: (edge.relationshipTypes ?? []).map((t) => RELATIONSHIP_TYPE_LABELS[t] ?? t).join(" / "),
      })),
      relationshipOptions: RELATIONSHIP_TYPE_OPTIONS.map((option) => ({ ...option, checked: selected.has(option.value) })),
      selectedRelationshipTypes: [...selected],
      memories: detail.recentMemories.map((m) => ({ id: String(m.id), content: m.content, type: m.type })),
      timeline: timeline.items.map((t) => ({ at: (t.occurredAt || "").slice(0, 10), title: t.title })),
    });
  },
  async brief() {
    const result = await page.client.missoryAssistant.assistant.briefings.create({ personId: this.personId });
    this.setData({ briefing: result.briefing });
  },
  startEdit() {
    const person = this.personDetail.person;
    this.setData({
      editing: true,
      note: "",
      editForm: { displayName: person.displayName ?? "", title: person.title ?? "", city: person.city ?? "" },
    });
  },
  cancelEdit() { this.setData({ editing: false }); },
  onEditField(event) {
    this.setData({ ["editForm." + event.currentTarget.dataset.field]: event.detail.value });
  },
  async saveEdit() {
    const displayName = this.data.editForm.displayName.trim();
    if (!displayName || !this.personDetail) return;
    const person = this.personDetail.person;
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
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "保存失败" });
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
    } catch (e) {
      this.setData({ note: (e && e.message) || "删除失败" });
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
      await page.client.missory.relationships.create(this.personId, { relationshipTypes: this.data.selectedRelationshipTypes });
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "保存失败" });
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
      this.refresh();
    } catch (e) {
      this.setData({ note: (e && e.message) || "解除失败" });
    }
  },
});
