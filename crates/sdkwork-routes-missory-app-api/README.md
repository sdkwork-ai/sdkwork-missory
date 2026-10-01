# sdkwork-routes-missory-app-api

App API HTTP surface for SDKWork Missory. Mounts the `missory` resources under the
locked app-api prefix `/app/v3/api` and maps service results to the standard
`SdkWorkApiResponse` / RFC 9457 problem envelopes. Handlers stay thin: parse, call
the `MissoryAppApi` port, map to the envelope.

Canonical specs: `../../../sdkwork-specs/API_SPEC.md`,
`../../../sdkwork-specs/RUST_CODE_SPEC.md` section 2.
