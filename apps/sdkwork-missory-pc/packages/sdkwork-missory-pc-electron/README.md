# @sdkwork/missory-pc-electron

Desktop host (Electron architecture family, APP_PC_ARCHITECTURE_SPEC naming
`sdkwork-<code>-pc-electron`). Wraps the built PC web console as a desktop app:

- loads `apps/sdkwork-missory-pc/dist/<profile>/<env>` (or `SDKWORK_DESKTOP_START_URL`),
- shell-only: contextIsolation + sandbox on, no business logic, no SDK imports,
- packaging evidence manifest at `target/desktop/desktop-host.manifest.json`;
  signed installers are produced by the release pipeline through `bin/`.

## Commands

```bash
pnpm --dir apps/sdkwork-missory-pc build:prod     # build the console first
pnpm --dir apps/sdkwork-missory-pc/packages/sdkwork-missory-pc-electron check
pnpm --dir apps/sdkwork-missory-pc/packages/sdkwork-missory-pc-electron package
```

Desktop dev run: `SDKWORK_DESKTOP_START_URL=http://127.0.0.1:3910 pnpm --dir
apps/sdkwork-missory-pc/packages/sdkwork-missory-pc-electron start` (requires
`pnpm install` at the PC app root so electron is downloaded).
