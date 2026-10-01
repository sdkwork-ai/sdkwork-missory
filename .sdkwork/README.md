# SDKWork Missory Workspace Metadata

This directory is the source-controlled SDKWork workspace metadata root for `sdkwork-missory`.

It is distinct from generated SDK output `.sdkwork/` directories. Generated output must not
store repository skills, plugins, runtime files, databases, logs, caches, or secrets.

Local-only subdirectories (`local/`, `tmp/`, `cache/`, `secrets/`, `manual-backups/`) are
git-ignored by `.sdkwork/.gitignore`.

Canonical standards:

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/COMPONENT_SPEC.md`
