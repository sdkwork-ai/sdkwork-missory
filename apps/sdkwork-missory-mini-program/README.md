# sdkwork-missory-mini-program

WeChat mini program for 念忆 · Missory: home digest, people, memories
(Fact ≠ Inference confirm workflow), and the draft-only AI assistant.

## Build & open

```bash
pnpm install                 # repository root
pnpm --dir apps/sdkwork-missory-mini-program build:mini-program
# then open apps/sdkwork-missory-mini-program in WeChat DevTools (project.config.json)
```

The bundle stamps `config/mini-program/runtime-env.<profileId>.json` values
(API base URL defaults to the standalone gateway). Point a real appid via
`project.config.json` / `config/host/mp-weixin.*.json`.
