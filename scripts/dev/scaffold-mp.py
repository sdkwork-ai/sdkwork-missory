#!/usr/bin/env python3
"""Scaffold apps/sdkwork-missory-mini-program (native WeChat + TS packages + esbuild)."""
import json
import os

BASE = "apps/sdkwork-missory-mini-program"

def w(path, content):
    full = os.path.join(BASE, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8", newline="\n") as f:
        f.write(content)

def j(path, data):
    w(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")

# ---------------- root ----------------
j("package.json", {
    "name": "@sdkwork/sdkwork-missory-mini-program",
    "private": True,
    "version": "0.1.0",
    "type": "module",
    "scripts": {
        "dev": "pnpm dev:standalone",
        "dev:standalone": "pnpm exec sdkwork-app dev --root ../.. --deployment-profile standalone",
        "dev:cloud": "pnpm exec sdkwork-app dev --root ../.. --deployment-profile cloud",
        "build": "node scripts/build-runtime.mjs",
        "build:mini-program": "node scripts/build-runtime.mjs",
        "build:mini-program:staging": "node scripts/build-runtime.mjs --deployment-profile cloud --environment staging",
        "build:mini-program:prod": "node scripts/build-runtime.mjs --deployment-profile cloud --environment production",
        "typecheck": "tsc --noEmit -p tsconfig.json",
        "test": "node --test tests/*.test.mjs",
        "check": "pnpm typecheck && pnpm test && pnpm build",
        "verify": "pnpm check"
    },
    "dependencies": {
        "@sdkwork/missory-mp-commons": "workspace:*",
        "@sdkwork/missory-mp-core": "workspace:*",
        "@sdkwork/missory-mp-host": "workspace:*"
    },
    "devDependencies": {
        "esbuild": "catalog:",
        "miniprogram-api-typings": "catalog:",
        "typescript": "catalog:",
        "@types/node": "catalog:"
    }
})

w("tsconfig.json", """{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "noEmit": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "types": ["miniprogram-api-typings"]
  },
  "include": ["src/**/*.ts", "packages/*/src/**/*.ts"]
}
""")

j("project.config.json", {
    "miniprogramRoot": "src/",
    "projectname": "sdkwork-missory-mini-program",
    "appid": "touristappid",
    "compileType": "miniprogram",
    "setting": {"es6": True, "postcss": True, "minified": True}
})

j("src/app.json", {
    "pages": [
        "pages/home/index",
        "pages/people/index",
        "pages/person/index",
        "pages/memories/index",
        "pages/assistant/index"
    ],
    "window": {
        "navigationBarTitleText": "念忆 · Missory",
        "navigationBarBackgroundColor": "#0f766e",
        "navigationBarTextStyle": "white"
    },
    "style": "v2",
    "sitemapLocation": "sitemap.json"
})

j("src/sitemap.json", {"rules": [{"action": "allow", "page": "*"}]})

w("src/app.js", """// 念忆 mini-program entry. The bundled runtime (dist/runtime/runtime.js)
// materializes config/mini-program runtime-env values before pages read them.
const runtime = require('../dist/runtime/runtime.js');

App({
  runtime,
  onLaunch() {
    runtime.bootstrap();
  },
});
""")

w("src/app.wxss", """page {
  background: #f6f7f9;
  color: #0f172a;
  font-size: 28rpx;
}

.page {
  padding: 24rpx;
  display: flex;
  flex-direction: column;
  gap: 24rpx;
}

.card {
  background: #ffffff;
  border-radius: 16rpx;
  padding: 24rpx;
  box-shadow: 0 2rpx 8rpx rgba(15, 23, 42, 0.06);
}

.card-title {
  font-size: 30rpx;
  font-weight: 600;
  margin-bottom: 16rpx;
}

.muted {
  color: #64748b;
  font-size: 24rpx;
}

.button {
  background: #0f766e;
  color: #ffffff;
  border-radius: 12rpx;
  padding: 12rpx 24rpx;
  font-size: 26rpx;
  display: inline-block;
}

.row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12rpx 0;
  border-bottom: 1rpx solid #e2e8f0;
}

.badge {
  display: inline-block;
  background: #e6f2f0;
  color: #0f766e;
  border-radius: 999rpx;
  padding: 2rpx 16rpx;
  font-size: 22rpx;
  margin-right: 8rpx;
}
""")

# ---------------- bootstrap (thin; bundle owns logic) ----------------
w("src/bootstrap/environment.ts", """import { resolveMissoryMpEnvironment } from "@sdkwork/missory-mp-core";

export function loadEnvironment() {
  // Values are injected at build time by scripts/build-runtime.mjs from
  // config/mini-program/runtime-env.<profile>.<environment>.json.
  return resolveMissoryMpEnvironment({
    environment: process.env.SDKWORK_MISSORY_ENVIRONMENT ?? "development",
    deploymentProfile: process.env.SDKWORK_MISSORY_DEPLOYMENT_PROFILE ?? "standalone",
    profileId: process.env.SDKWORK_MISSORY_PROFILE_ID ?? "standalone.development",
    appApiBaseUrl: process.env.SDKWORK_MISSORY_MP_APP_API_BASE_URL ?? "http://127.0.0.1:8460",
  });
}
""")

w("src/bootstrap/runtimeBundle.ts", """// The single safe runtime module bundled by esbuild into src/runtime/runtime.js
// (MINI_PROGRAM_APP_ARCHITECTURE_SPEC section 5): host adapter registration,
// SDK client construction, and services exposed to page JS through one object.
import { bootstrapMissoryMpRuntime } from "@sdkwork/missory-mp-core";

const env = {
  environment: "__SDKWORK_MISSORY_ENVIRONMENT__",
  deploymentProfile: "__SDKWORK_MISSORY_DEPLOYMENT_PROFILE__",
  profileId: "__SDKWORK_MISSORY_PROFILE_ID__",
  appApiBaseUrl: "__SDKWORK_MISSORY_MP_APP_API_BASE_URL__",
};

const runtime = bootstrapMissoryMpRuntime(env);

export function bootstrap() {
  return runtime;
}

export default runtime;
""")

# ---------------- packages: host ----------------
j("packages/sdkwork-missory-mp-host/package.json", {
    "name": "@sdkwork/missory-mp-host",
    "version": "0.1.0", "private": True, "type": "module",
    "main": "./src/index.ts", "types": "./src/index.ts",
    "exports": {".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"}}
})
w("packages/sdkwork-missory-mp-host/src/index.ts", """export { createWxFetchAdapter } from "./wx-fetch-adapter.ts";
export { showToast, type ToastKind } from "./toast.ts";
""")

w("packages/sdkwork-missory-mp-host/src/wx-fetch-adapter.ts", """/**
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
""")

w("packages/sdkwork-missory-mp-host/src/toast.ts", """export type ToastKind = "ok" | "error";

export function showToast(title: string, kind: ToastKind = "ok"): void {
  const wx = (globalThis as { wx?: { showToast?: (options: { title: string; icon: string }) => void } }).wx;
  if (typeof wx?.showToast !== "function") return;
  wx.showToast({ title, icon: kind === "ok" ? "success" : "none" });
}
""")

w("packages/sdkwork-missory-mp-host/README.md", """# @sdkwork/missory-mp-host

WeChat host adapters for the missory mini program: the wx.request→fetch bridge
consumed by the generated SDK transport, plus toast. Platform capabilities only
surface through typed adapters with stable outcomes.
""")

# ---------------- packages: commons ----------------
j("packages/sdkwork-missory-mp-commons/package.json", {
    "name": "@sdkwork/missory-mp-commons",
    "version": "0.1.0", "private": True, "type": "module",
    "main": "./src/index.ts", "types": "./src/index.ts",
    "exports": {".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"}}
})
w("packages/sdkwork-missory-mp-commons/src/index.ts", """export { zhCn } from "./i18n/zh-CN/missory/labels.ts";
export { memoryTypeLabel, relationshipTypeLabel } from "./labels.ts";
""")

os.makedirs(os.path.join(BASE, "packages/sdkwork-missory-mp-commons/src/i18n/zh-CN/missory"), exist_ok=True)
w("packages/sdkwork-missory-mp-commons/src/i18n/zh-CN/missory/labels.ts", """export const zhCn = {
  homeTitle: "今天，有谁值得你想起？",
  peopleTitle: "人物",
  memoriesTitle: "记忆",
  assistantTitle: "AI 社交助手",
  remindersTitle: "今日关系",
  recentMemories: "最近记忆",
  recentPersons: "最近人物",
  confirm: "确认",
  reject: "拒绝",
  dismiss: "忽略",
  empty: "暂无内容",
  draftOnly: "仅生成草稿，不会自动发送。",
} as const;
""")

w("packages/sdkwork-missory-mp-commons/src/labels.ts", """export function memoryTypeLabel(value: string): string {
  const map: Record<string, string> = {
    semantic: "事实", episodic: "经历", temporal: "近况",
    relationship: "关系", preference: "偏好", commitment: "承诺",
  };
  return map[value] ?? value;
}

export function relationshipTypeLabel(value: string): string {
  const map: Record<string, string> = {
    family: "家人", friend: "朋友", classmate: "同学", colleague: "同事",
    client: "客户", partner: "合作伙伴", teacher: "老师", student: "学生",
    neighbor: "邻居", spouse: "伴侣", other: "其他",
  };
  return map[value] ?? value;
}
""")

w("packages/sdkwork-missory-mp-commons/README.md", """# @sdkwork/missory-mp-commons

Mini-program shared locale fragments (src/i18n/<locale>/...) and label helpers.
""")

# ---------------- packages: core ----------------
j("packages/sdkwork-missory-mp-core/package.json", {
    "name": "@sdkwork/missory-mp-core",
    "version": "0.1.0", "private": True, "type": "module",
    "main": "./src/index.ts", "types": "./src/index.ts",
    "exports": {".": {"types": "./src/index.ts", "import": "./src/index.ts", "default": "./src/index.ts"}},
    "dependencies": {
        "@sdkwork/missory-app-sdk": "workspace:*",
        "@sdkwork/missory-mp-host": "workspace:*",
        "@sdkwork/missory-mp-commons": "workspace:*"
    }
})

w("packages/sdkwork-missory-mp-core/src/index.ts", """export { bootstrapMissoryMpRuntime, resolveMissoryMpEnvironment } from "./runtime.ts";
export type { MissoryMpEnvironment, MissoryMpRuntime } from "./runtime.ts";
""")

w("packages/sdkwork-missory-mp-core/src/runtime.ts", """import { createClient, type SdkworkAppClient } from "@sdkwork/missory-app-sdk";

import { createWxFetchAdapter } from "@sdkwork/missory-mp-host";

export interface MissoryMpEnvironment {
  environment: string;
  deploymentProfile: string;
  profileId: string;
  appApiBaseUrl: string;
}

export interface MissoryMpRuntime {
  environment: MissoryMpEnvironment;
  client: SdkworkAppClient;
}

export function resolveMissoryMpEnvironment(input: MissoryMpEnvironment): MissoryMpEnvironment {
  if (!input.appApiBaseUrl.startsWith("http")) {
    throw new Error("missory mp: appApiBaseUrl must be an absolute origin");
  }
  return input;
}

export function bootstrapMissoryMpRuntime(environment: MissoryMpEnvironment): MissoryMpRuntime {
  const wxFetch = createWxFetchAdapter();
  (globalThis as { fetch?: unknown }).fetch = wxFetch as unknown as typeof fetch;
  const client = createClient({ baseUrl: environment.appApiBaseUrl, platform: "mini-program" });
  return { environment, client };
}
""")

w("packages/sdkwork-missory-mp-core/README.md", """# @sdkwork/missory-mp-core

Mini-program core: environment resolution, the generated app-sdk client
(transport bridged to wx.request through the host adapter), and services.
Constructs SDK clients; pages consume this runtime only.
""")

print("mini-program base tree written")
