# apps/sdkwork-missory-h5/etc

Source config for the PC browser surface: the component deployment index
(`sdkwork.deployment.config.json`), browser runtime bindings
(`browser.runtime.json`), and the ten checked-in runtime-env source documents
under `browser/`.

Real secrets never live here (`SOURCE_CONFIG_SPEC.md`). Materialization writes
`public/runtime-env.json` (git-ignored) via `scripts/materialize-runtime-env.mjs`.
