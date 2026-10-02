#!/usr/bin/env python3
"""MP pages WXML/WXSS with real bindings + build script + config + manifest."""
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

w("src/pages/home/index.wxml", """<view class="page">
  <view class="card">
    <view class="card-title">今天，有谁值得你想起？</view>
    <view wx:if="{{loading}}" class="muted">加载中…</view>
    <view wx:elif="{{error}}" class="muted">{{error}}</view>
    <block wx:else>
      <view wx:for="{{reminders}}" wx:key="reminderId" class="row">
        <view><strong>{{item.name}}</strong> <text class="muted">{{item.title}}</text> <text class="badge">{{item.type}}</text></view>
        <view class="button" bindtap="dismiss" data-id="{{item.reminderId}}">忽略</view>
      </view>
      <view wx:if="{{reminders.length === 0}}" class="muted">暂无提醒</view>
    </block>
  </view>
  <view class="card">
    <view class="card-title">最近记忆</view>
    <view wx:for="{{memories}}" wx:key="id" class="row">{{item.content}}</view>
    <view wx:if="{{memories.length === 0}}" class="muted">还没有记忆</view>
  </view>
  <view class="card">
    <view class="card-title">最近人物</view>
    <view wx:for="{{persons}}" wx:key="id" class="row">
      <navigator url="/pages/person/index?id={{item.id}}">{{item.name}}</navigator>
    </view>
    <view wx:if="{{persons.length === 0}}" class="muted">还没有人物</view>
  </view>
</view>
""")

w("src/pages/people/index.wxml", """<view class="page">
  <view class="card">
    <view class="card-title">人物</view>
    <view class="row">
      <input value="{{keyword}}" bindinput="onKeyword" placeholder="搜索姓名/标签/公司" />
      <view class="button" bindtap="onSearch">搜索</view>
    </view>
    <view wx:if="{{loading}}" class="muted">加载中…</view>
    <view wx:for="{{persons}}" wx:key="id" class="row">
      <navigator url="/pages/person/index?id={{item.id}}">{{item.name}}</navigator>
      <text class="muted">{{item.title}}</text>
    </view>
    <view wx:if="{{persons.length === 0 && !loading}}" class="muted">还没有人物</view>
  </view>
</view>
""")

w("src/pages/person/index.wxml", """<view class="page">
  <view class="card" wx:if="{{person}}">
    <view class="card-title">{{person.name}}</view>
    <view class="muted">{{person.title}} {{person.city}}</view>
    <view class="row">认识天数：{{person.knownDays}}</view>
    <view class="row">记忆 {{person.memoryCount}} 条</view>
    <view class="button" bindtap="brief">生成见面简报</view>
    <view wx:if="{{briefing}}" style="margin-top: 16rpx; white-space: pre-wrap;">{{briefing}}</view>
  </view>
  <view class="card">
    <view class="card-title">重要记忆</view>
    <view wx:for="{{memories}}" wx:key="id" class="row"><text class="badge">{{item.type}}</text>{{item.content}}</view>
  </view>
  <view class="card">
    <view class="card-title">时间线</view>
    <view wx:for="{{timeline}}" wx:key="at" class="row"><text class="muted">{{item.at}}</text> {{item.title}}</view>
  </view>
</view>
""")

w("src/pages/memories/index.wxml", """<view class="page">
  <view class="card">
    <view class="card-title">从文本提取候选记忆</view>
    <textarea value="{{text}}" bindinput="onText" placeholder="粘贴对话或随笔…" style="width: 100%; min-height: 120rpx;" />
    <view class="button" bindtap="extract" style="margin-top: 12rpx;">提取候选</view>
    <view wx:if="{{note}}" class="muted">{{note}}</view>
  </view>
  <view class="card">
    <view class="card-title">记忆列表</view>
    <view wx:if="{{loading}}" class="muted">加载中…</view>
    <view wx:for="{{memories}}" wx:key="id" class="row">
      <view>
        <text class="badge">{{item.type}}</text>
        <text class="badge">{{item.origin === 'inference' ? '推断' : '事实'}}</text>
        {{item.content}}
      </view>
      <view wx:if="{{item.status === 'candidate'}}" style="display: flex; gap: 12rpx;">
        <view class="button" bindtap="confirm" data-id="{{item.id}}">确认</view>
        <view class="button" style="background: #64748b;" bindtap="reject" data-id="{{item.id}}">拒绝</view>
      </view>
    </view>
    <view wx:if="{{memories.length === 0 && !loading}}" class="muted">没有匹配的记忆</view>
  </view>
</view>
""")

w("src/pages/assistant/index.wxml", """<view class="page">
  <view class="card">
    <view class="card-title">问我任何关于你人际关系的问题</view>
    <input value="{{question}}" bindinput="onQuestion" placeholder="李明是谁？" />
    <view class="button" bindtap="ask" style="margin-top: 12rpx;">发送</view>
    <view wx:if="{{answer}}" style="margin-top: 16rpx; white-space: pre-wrap;">{{answer}}</view>
  </view>
  <view class="card">
    <view class="card-title">消息草稿（仅草稿，不会自动发送）</view>
    <input value="{{personId}}" bindinput="onPersonId" placeholder="人物 ID" />
    <view class="button" bindtap="draft" style="margin-top: 12rpx;">生成生日祝福草稿</view>
    <view wx:if="{{draft}}" style="margin-top: 16rpx; white-space: pre-wrap;">{{draft}}</view>
  </view>
</view>
""")

# build-runtime.mjs: validate env doc, esbuild bundle, stamp manifest
w("scripts/build-runtime.mjs", """#!/usr/bin/env node
// Bundles src/bootstrap/runtimeBundle.ts into src/runtime/runtime.js with the
// selected config/mini-program runtime-env values stamped in (MINI_PROGRAM_
// APP_ARCHITECTURE_SPEC section 5: one safe runtime module before upload).
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function option(argv, name, fallback) {
  const index = argv.indexOf(name);
  return index >= 0 ? argv[index + 1] : fallback;
}

const argv = process.argv.slice(2);
const deploymentProfile = option(argv, "--deployment-profile", "standalone");
const environment = option(argv, "--environment", "development");
const profileId = `${deploymentProfile}.${environment}`;

const envFile = path.join(appRoot, "config/mini-program", `runtime-env.${profileId}.json`);
if (!fs.existsSync(envFile)) {
  process.stderr.write(`runtime-env document missing: ${envFile}\\n`);
  process.exit(2);
}
const doc = JSON.parse(fs.readFileSync(envFile, "utf8"));
for (const key of ["SDKWORK_ENVIRONMENT", "SDKWORK_DEPLOYMENT_PROFILE", "SDKWORK_PROFILE_ID", "SDKWORK_RUNTIME_TARGET"]) {
  if (!doc[key]) {
    process.stderr.write(`runtime-env document ${profileId} misses ${key}\\n`);
    process.exit(2);
  }
}

const define = {
  "__SDKWORK_MISSORY_ENVIRONMENT__": JSON.stringify(doc.SDKWORK_ENVIRONMENT),
  "__SDKWORK_MISSORY_DEPLOYMENT_PROFILE__": JSON.stringify(doc.SDKWORK_DEPLOYMENT_PROFILE),
  "__SDKWORK_MISSORY_PROFILE_ID__": JSON.stringify(doc.SDKWORK_PROFILE_ID),
  "__SDKWORK_MISSORY_MP_APP_API_BASE_URL__": JSON.stringify(doc.SDKWORK_MISSORY_MP_APP_API_BASE_URL),
};

const outfile = path.join(appRoot, "src/runtime/runtime.js");
await build({
  entryPoints: [path.join(appRoot, "src/bootstrap/runtimeBundle.ts")],
  bundle: true,
  platform: "neutral",
  format: "cjs",
  target: "es2020",
  outfile,
  define,
  alias: {
    "@sdkwork/missory-mp-core": path.join(appRoot, "packages/sdkwork-missory-mp-core/src/index.ts"),
    "@sdkwork/missory-mp-host": path.join(appRoot, "packages/sdkwork-missory-mp-host/src/index.ts"),
    "@sdkwork/missory-mp-commons": path.join(appRoot, "packages/sdkwork-missory-mp-commons/src/index.ts"),
    "@sdkwork/missory-app-sdk": path.join(appRoot, "../../sdks/sdkwork-missory-app-sdk/sdkwork-missory-app-sdk-typescript/src/index.ts"),
  },
  external: ["@sdkwork/sdk-common", "@sdkwork/utils"],
});

fs.writeFileSync(
  path.join(appRoot, "src/runtime/build-manifest.json"),
  JSON.stringify({
    schemaVersion: 1,
    kind: "sdkwork.mini-program.runtime-manifest",
    application: "sdkwork-missory-mini-program",
    profileId,
    entry: "runtime.js",
    builtAt: "materialized",
  }, null, 2) + "\\n",
);
process.stdout.write(`[sdkwork-missory-mp] runtime bundled for ${profileId} -> src/runtime/runtime.js\\n`);
""")

# runtime-env docs (10)
def env_doc(profile, environment, base):
    return {
        "SDKWORK_ENVIRONMENT": environment,
        "SDKWORK_DEPLOYMENT_PROFILE": profile,
        "SDKWORK_PROFILE_ID": f"{profile}.{environment}",
        "SDKWORK_RUNTIME_TARGET": "mini-program",
        "SDKWORK_MISSORY_MP_APP_API_BASE_URL": base,
    }

sources = {
    "standalone.development": "http://127.0.0.1:8460",
    "standalone.test": "http://127.0.0.1:8461",
    "standalone.staging": "https://missory-staging.sdkwork.com",
    "standalone.demo": "https://missory-demo.sdkwork.com",
    "standalone.production": "https://missory.sdkwork.com",
    "cloud.development": "https://missory-dev.sdkwork.com",
    "cloud.test": "https://missory-test.sdkwork.com",
    "cloud.staging": "https://missory-staging.sdkwork.com",
    "cloud.demo": "https://missory-demo.sdkwork.com",
    "cloud.production": "https://missory.sdkwork.com",
}
for key, base in sources.items():
    j(f"config/mini-program/runtime-env.{key}.json", env_doc(*key.split("."), base))

for env in ["development", "test", "staging", "production"]:
    j(f"config/host/mp-weixin.{env}.example.json", {
        "appid": "touristappid",
        "projectname": f"sdkwork-missory-mini-program-{env}",
        "description": f"WeChat devtools project profile for {env}. Copy to project.config.json with the real appid."
    })

j("etc/sdkwork.deployment.config.json", {
    "schemaVersion": 1,
    "kind": "sdkwork.component-deployment",
    "application": "sdkwork-missory-mini-program",
    "parentDeploymentConfig": "../../../etc/sdkwork.deployment.config.json",
    "parentTopologySpec": "../../../specs/topology.spec.json",
    "runtimeConfig": None,
    "runtimeTarget": "mini-program",
    "profiles": {key: {"source": f"config/mini-program/runtime-env.{key}.json"} for key in sources},
    "materialization": {
        "authority": "../../../etc/sdkwork.deployment.config.json",
        "command": "node scripts/build-runtime.mjs",
        "format": "mini-program-runtime-bundle",
        "output": "../src/runtime/runtime.js",
        "checkMode": "--check"
    }
})
w("etc/README.md", """# apps/sdkwork-missory-mini-program/etc

Component deployment index + mini-program runtime-env source documents
(config/mini-program). Real secrets never live here.
""")

print("mp pages/build/config written")
