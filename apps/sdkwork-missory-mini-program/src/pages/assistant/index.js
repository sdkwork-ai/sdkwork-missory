const { page } = getApp().runtime;
Page({
  data: { question: "", answer: "", draft: "", personId: "" },
  onQuestion(event) { this.setData({ question: event.detail.value }); },
  async ask() {
    if (!this.data.question.trim()) return;
    const result = await page.client.missoryAssistant.assistant.query({ question: this.data.question });
    this.setData({ answer: result.answer });
  },
  onPersonId(event) { this.setData({ personId: event.detail.value }); },
  async draft() {
    if (!this.data.personId) return;
    const result = await page.client.missoryAssistant.assistant.messageDrafts.create({ personId: this.data.personId, scenario: "birthday", tone: "warm" });
    this.setData({ draft: result.draft + "

" + result.disclaimer });
  },
});
