// Host adapter seam — re-exports the typed wx bridges from mp-host. Pages must
// never call wx.* directly (MINI_PROGRAM_APP_ARCHITECTURE_SPEC host-adapter rule).
export { createWxFetchAdapter, showToast } from "@sdkwork/missory-mp-host";
