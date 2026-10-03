import type { InputEvent, MissoryAppInstance } from "../../typings/runtime";

const { page } = getApp<MissoryAppInstance>().runtime;

type ChatSummaryView = {
  summary: string;
  candidates: Array<{ id: string; type: string; content: string }>;
};

type AssistantData = {
  question: string;
  answer: string;
  draft: string;
  personId: string;
  chatPersonId: string;
  chatText: string;
  chatSummary: ChatSummaryView | null;
};

type AssistantCustom = {
  onQuestion(event: InputEvent): void;
  ask(): Promise<void>;
  onPersonId(event: InputEvent): void;
  draft(): Promise<void>;
  onChatPersonId(event: InputEvent): void;
  onChatText(event: InputEvent): void;
  summarizeChat(): Promise<void>;
};

Page<AssistantData, AssistantCustom>({
  data: {
    question: "",
    answer: "",
    draft: "",
    personId: "",
    chatPersonId: "",
    chatText: "",
    chatSummary: null,
  },
  onQuestion(event) {
    this.setData({ question: event.detail.value });
  },
  async ask() {
    if (!this.data.question.trim()) return;
    const result = await page.client.missoryAssistant.assistant.query({ question: this.data.question });
    this.setData({ answer: result.answer });
  },
  onPersonId(event) {
    this.setData({ personId: event.detail.value });
  },
  async draft() {
    if (!this.data.personId) return;
    const result = await page.client.missoryAssistant.assistant.messageDrafts.create({
      personId: this.data.personId,
      scenario: "birthday",
      tone: "warm",
    });
    this.setData({ draft: `${result.draft}\n\n${result.disclaimer}` });
  },
  onChatPersonId(event) {
    this.setData({ chatPersonId: event.detail.value });
  },
  onChatText(event) {
    this.setData({ chatText: event.detail.value });
  },
  async summarizeChat() {
    const personId = this.data.chatPersonId.trim();
    const text = this.data.chatText.trim();
    if (!personId || !text) return;
    const result = await page.client.missoryAssistant.assistant.chatSummaries.create({ personId, text });
    this.setData({
      chatSummary: {
        summary: result.summary,
        candidates: (result.candidateMemories ?? []).map((memory) => ({
          id: String(memory.id),
          type: memory.type,
          content: memory.content,
        })),
      },
    });
  },
});
