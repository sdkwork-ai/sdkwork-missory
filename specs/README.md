# sdkwork-missory Repository Specs

This directory holds the repository-wide machine-readable contracts.

| File | Purpose |
| --- | --- |
| `component.spec.json` | Root component contract for `sdkwork-missory` (COMPONENT_SPEC.md) |
| `topology.spec.json` | Runtime connectivity topology (APP_RUNTIME_TOPOLOGY_SPEC.md v5) |

Global standards remain authoritative: `../sdkwork-specs/README.md`. Each crate and plugin
owns its own `specs/component.spec.json`.

Verification:

```bash
node ../sdkwork-specs/tools/verify-repo.mjs --root .
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root . --spec specs/topology.spec.json
```
