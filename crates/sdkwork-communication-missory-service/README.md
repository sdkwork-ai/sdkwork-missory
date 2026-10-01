# sdkwork-communication-missory-service

Missory domain service implementing `sdkwork_missory_contract::ports::MissoryAppApi`
on top of the `sdkwork-missory-spi` store ports. Owns business rules: person and
relationship management, memory lifecycle with Fact ≠ Inference enforcement,
reminder derivation, story summaries, and the draft-only rule-based social agent
with a pluggable `SocialTextModel`.

Canonical specs: `../../../sdkwork-specs/RUST_CODE_SPEC.md`,
`../../../sdkwork-specs/API_SPEC.md`.
