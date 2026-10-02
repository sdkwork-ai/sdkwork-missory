// 念忆 mini-program entry. The bundled runtime (src/runtime/runtime.js,
// materialized by `pnpm build:mini-program`) registers the host adapters and
// the IAM dual-token SDK client before pages read it.
const runtimeModule = require('./runtime/runtime.js');

App({
  runtime: runtimeModule,
  onLaunch() {
    const app = runtimeModule.bootstrap();
    // Session gate: non-development environments require a signed-in
    // dual-token session; development rides the gateway dev bypass.
    if (app.environment.environment !== "development" && !app.session.isAuthenticated()) {
      wx.reLaunch({ url: '/pages/login/index' });
    }
  },
});
