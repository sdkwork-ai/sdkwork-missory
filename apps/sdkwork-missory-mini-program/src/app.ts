// 念忆 mini-program entry. The bundled runtime (src/runtime/runtime.js,
// materialized by `pnpm build:mini-program`) registers the host adapters and
// the IAM dual-token SDK client before pages read it.
import type * as missoryRuntimeBundle from "./bootstrap/runtimeBundle";
import type { MissoryAppInstance } from "./typings/runtime";

// WeChat's module system resolves CommonJS `require` only, so the entry loads
// the generated runtime artifact through a require call typed against the
// bootstrap sources.
declare function require(moduleName: "./runtime/runtime.js"): typeof missoryRuntimeBundle;

App<MissoryAppInstance>({
  runtime: require("./runtime/runtime.js"),
  onLaunch() {
    const runtime = this.runtime.bootstrap();
    // Session gate: non-development environments require a signed-in
    // dual-token session; development rides the gateway dev bypass.
    if (runtime.environment.environment !== "development" && !runtime.session.isAuthenticated()) {
      wx.reLaunch({ url: "/pages/login/index" });
    }
  },
});
