# 念忆 · Missory — Product Requirements (PRD)

- Status: active
- Owner: sdkwork-missory platform team
- Application: sdkwork-missory (application code `missory`)
- Updated: 2026-10-01
- Specs: `../../../sdkwork-specs/REQUIREMENTS_SPEC.md`, `../../../sdkwork-specs/API_SPEC.md`

Source: adapted from the attached PRD《念忆 · Missory AI 个人关系记忆产品 PRD V1.0》. This document
is the canonical product baseline for engineering; requirement changes go through
`REQUIREMENTS_SPEC.md` change control and land as `REQ-*` working documents.

## 1. Product Definition

**念忆 · Missory** is an AI personal social memory product centered on 人 (people), 关系
(relationships), 记忆 (memories), and 故事 (stories). It is not an address book, not a
knowledge base, and not a CRM. It helps a user build:

- a personal profile and person profiles,
- a personal relationship graph,
- long-term structured memory,
- shared experiences and person stories,
- relationship reminders, and an AI social context.

Slogan: **记住每一个重要的人。**

## 2. Core Value Chain

```text
People → Relationship → Memory → Story → AI Agent
```

1. Remember people.
2. Understand relationships.
3. Save memories.
4. Organize stories.
5. Help act — AI assists, the user decides.

## 3. User Problems Addressed

- Too many people, details forgotten (when/where met, last conversation, promises made).
- Fragmented information across contacts, chats, mail, calendar, photos, and the user's own head.
- Meeting someone again without context.
- Important relationships silently neglected (birthdays, promises, long silences).

## 4. Target Users

High-social-frequency users, founders/managers, creators, client-facing professionals, and
everyday users who want to remember family and friends.

## 5. Core Domain Objects

```text
User → Person → {Profile, Memory, Event, Preference, Conversation, Story}
User → Relationship (person ↔ person, typed, dated)
```

Core object set: `Person`, `Relationship`, `Memory`, `Event`, `Story`, `Conversation`,
`Task`, plus `Reminder`, `Commitment`, and `Source` (provenance).

## 6. P0 (MVP) Functional Scope

### 6.1 People (人物)

- Create / edit / delete a person (P0: soft delete + hard delete of owned data).
- Person search (keyword over name, alias, title, tags, notes).
- Person profile view: base info, relationships, about, interests, preferences, recent
  activity, important memories, shared experiences, commitments, timeline, AI suggestions.

### 6.2 Relationships (关系)

- Relationship types: family, friend, classmate, colleague, client, partner, teacher,
  student, neighbor, spouse/partner, other — multiple types per relationship.
- Relationship attributes: types, met-at time (start), last-contact time, description,
  importance, user notes.
- Per-person relationship timeline (meet → events → recent contact).

### 6.3 Memory (记忆)

- Manual memory records and AI-extracted memories from free text.
- Memory types (per PRD §17): semantic (fact), episodic (experience), temporal
  (time-varying), relationship, preference, commitment.
- **Fact ≠ Inference (P0 design requirement):** every memory records
  `origin = fact | inference`; inferences additionally carry `confidence`, `source`,
  `reason`. Users confirm / edit / reject / dismiss inferences; AI inferences are never
  silently promoted to facts.
- Memory lifecycle: `candidate → confirmed → (rejected | archived | expired)`.
- Memory search (keyword + person + type + status filters), edit, delete, source inspection.

### 6.4 AI (Social Agent, draft-only)

- Person Q&A, memory Q&A, relationship Q&A (who is X, how long known, what did we discuss).
- Person summary; meeting briefing (明天见 TA 聊什么) composed from profile + relationship +
  memories + commitments.
- Chat summary from pasted conversation text (extract people, events, memories as
  **candidates** for confirmation).
- Message drafts (birthday greeting, check-in) in styles: brief / natural / warm / humorous /
  formal / friendly. **Only drafts are generated; nothing is ever auto-sent.**

### 6.5 Reminders (提醒)

- Derived reminders: long-uncontacted (per-person custom cycle), birthday approaching,
  anniversary, pending commitments, important life changes.
- Reminder actions: dismiss, snooze, adjust cycle. Reminders never trigger third-party contact.

### 6.6 Stories (故事)

- Create a story bundling a time range, participants, events, and memories.
- Story timeline view and AI story summary.

## 7. MVP Explicitly NOT Doing

No auto-sending messages, no stranger social, no public social network, no public person
ranking/leaderboards, no unauthorized third-party data collection, no complex CRM, no
automatic decision on behalf of the user to contact someone.

## 8. Roadmap

- **V2:** mail/calendar/chat import (authorized), voice, photos, automatic person/event
  recognition, proactive reminders, annual report, multi-device sync.
- **V3:** personal social graph — entity disambiguation, relationship-change detection,
  Life Agent, auto story generation, multimodal memory.
- **V4:** Personal AI — self + people + relationships + memory + life + agent.

## 9. Privacy And AI Safety (P0)

- Users own their data; every important record tracks `source_type`, `source_id`,
  `created_by`, `confidence`.
- Data visibility is `private` by default; sharing is explicit.
- AI must not: present guesses as facts, fabricate experiences, silently mutate core person
  info, contact third parties without authorization, leak other people's private data,
  cross user boundaries, merge different people, or state person conclusions without a source.
- Data deletion and export are supported.

## 10. Core Metrics

- Activation: profile created + first person + first memory.
- North star: number of distinct people actively recalled, queried, or updated per week.
- Guardrails: per-user people/memories/relationships, weekly AI queries, memory add/recall
  rate, reminder click-through and follow-through.

## 11. Acceptance Criteria (Engineering P0 Baseline)

1. All P0 operations above are exposed on the Missory app-api surface
   (`/app/v3/api/missory/*`) following `API_SPEC.md` §15.4 operation semantics.
2. Fact/inference separation is enforced at data, service, and API layers.
3. Reminder derivation covers long-uncontacted, birthday, commitment, and important events
   with dismiss/snooze state.
4. The rule-based assistant answers person/relationship/memory questions and produces
   briefings and drafts without any external model dependency (pluggable
   `LanguageModelPort` allows a real provider later).
5. `cargo test --workspace` and the repository standards gates pass.
