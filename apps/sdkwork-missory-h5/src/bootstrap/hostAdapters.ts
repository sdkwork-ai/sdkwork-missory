// H5 host adapter seam (APP_MOBILE_REACT_UI_SPEC: native concerns behind typed
// host adapters with stable error kinds). The browser host needs no native
// bridges today; Capacitor hosts register their adapters here.
export type HostAdapterOutcome<T> =
  | { status: "ok"; value: T }
  | { status: "unsupported" | "unavailable" | "cancelled" | "invalid-state"; detail?: string };

export interface MissoryH5HostAdapter {
  shareText(text: string): Promise<HostAdapterOutcome<null>>;
}

export const browserHostAdapter: MissoryH5HostAdapter = {
  async shareText(text: string) {
    if (typeof navigator !== "undefined" && typeof navigator.clipboard?.writeText === "function") {
      await navigator.clipboard.writeText(text);
      return { status: "ok", value: null };
    }
    return { status: "unsupported", detail: "clipboard is unavailable in this browser" };
  },
};
