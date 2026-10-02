/**
 * WeChat host network adapter: bridges the generated SDK's `fetch` transport to
 * `wx.request` (MINI_PROGRAM_APP_ARCHITECTURE_SPEC host-adapter rule; business
 * code stays on the generated SDK, never raw wx.request).
 */
export type WxFetch = (input: string, init?: { method?: string; headers?: Record<string, string>; body?: string }) => Promise<{
  ok: boolean;
  status: number;
  headers: Record<string, string>;
  text: () => Promise<string>;
}>;

interface WxRequestOptions {
  url: string;
  method?: string;
  header?: Record<string, string>;
  data?: string;
  success: (result: { statusCode: number; header?: Record<string, unknown>; data?: unknown }) => void;
  fail: (error: { errMsg: string }) => void;
}

type WxLike = {
  request: (options: WxRequestOptions) => void;
};

function wxLike(): WxLike {
  const candidate = (globalThis as Record<string, unknown>).wx as WxLike | undefined;
  if (!candidate || typeof candidate.request !== "function") {
    throw new Error("wx.request is unavailable; the WeChat host adapter requires the WeChat runtime");
  }
  return candidate;
}

export function createWxFetchAdapter(): WxFetch {
  const wx = wxLike();
  return async (input, init) => {
    const method = (init?.method ?? "GET").toUpperCase();
    const result = await new Promise<{ statusCode: number; header?: Record<string, unknown>; data?: unknown }>(
      (resolve, reject) => {
        wx.request({
          url: input,
          method,
          header: init?.headers,
          data: init?.body,
          success: resolve,
          fail: (error) => reject(new Error(error.errMsg ?? "wx.request failed")),
        });
      },
    );
    const headers: Record<string, string> = {};
    for (const [key, value] of Object.entries(result.header ?? {})) {
      headers[key.toLowerCase()] = String(value);
    }
    const raw = typeof result.data === "string" ? result.data : JSON.stringify(result.data ?? {});
    return {
      ok: result.statusCode >= 200 && result.statusCode < 300,
      status: result.statusCode,
      headers,
      text: async () => raw,
    };
  };
}
