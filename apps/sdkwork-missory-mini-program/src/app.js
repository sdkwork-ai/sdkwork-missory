// 念忆 mini-program entry. The bundled runtime (dist/runtime/runtime.js)
// materializes config/mini-program runtime-env values before pages read them.
const runtime = require('../dist/runtime/runtime.js');

App({
  runtime,
  onLaunch() {
    runtime.bootstrap();
  },
});
