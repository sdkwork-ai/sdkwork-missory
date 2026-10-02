# sdkwork-missory-h5

PC browser console for 念忆 · Missory (AI personal social memory): home digest,
people, memories (Fact ≠ Inference confirm workflow), and the draft-only social
AI assistant.

## Commands

```bash
pnpm install            # from the repository root
pnpm dev                # standalone topology (gateway + this renderer)
pnpm build:prod         # canonical build runner → dist/standalone/prod
pnpm check              # typecheck + tests
```

The app-api base URL resolves from `public/runtime-env.json` (materialized per
build from `etc/browser/runtime-env.*.json`). Phase-1 standalone development
uses the gateway's dev identity bypass (`SDKWORK_MISSORY_DEV_AUTH_BYPASS`).
