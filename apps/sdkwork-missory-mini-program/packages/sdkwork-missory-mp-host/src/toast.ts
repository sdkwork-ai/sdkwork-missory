export type ToastKind = "ok" | "error";

export function showToast(title: string, kind: ToastKind = "ok"): void {
  const wx = (globalThis as { wx?: { showToast?: (options: { title: string; icon: string }) => void } }).wx;
  if (typeof wx?.showToast !== "function") return;
  wx.showToast({ title, icon: kind === "ok" ? "success" : "none" });
}
