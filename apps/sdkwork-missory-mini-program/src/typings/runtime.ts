// Page-facing typing for the App() instance (MINI_PROGRAM_APP_ARCHITECTURE_
// SPEC: authored mini program sources are TypeScript; page JS is forbidden).
// `app.runtime` is the bundled runtime module — src/runtime/runtime.js,
// materialized by `pnpm build:mini-program` — so pages read every service
// through it and never import package internals directly. All imports of this
// module from app/page sources are `import type`: type-only syntax is erased
// by the WeChat DevTools TypeScript compiler plugin, so the emitted module
// graph keeps using the CommonJS runtime require.
import type * as missoryRuntimeBundle from "../bootstrap/runtimeBundle";

/** Shape of the registered App() instance pages read through `getApp()`. */
export type MissoryAppInstance = {
  runtime: typeof missoryRuntimeBundle;
};

/** Tap-style event carrying `data-id` from the page markup. */
export type TapEvent<Dataset extends WechatMiniprogram.IAnyObject = WechatMiniprogram.IAnyObject> =
  WechatMiniprogram.CustomEvent<Record<string, never>, WechatMiniprogram.IAnyObject, Dataset>;

/** `input`/`textarea` `bindinput` event. */
export type InputEvent = WechatMiniprogram.CustomEvent<{ value: string }>;

/** `picker mode="selector"` `bindchange` event (index value). */
export type SelectorChangeEvent = WechatMiniprogram.CustomEvent<{ value: string | number }>;

/** `picker mode="date"` `bindchange` event (`YYYY-MM-DD` value). */
export type DateChangeEvent = WechatMiniprogram.CustomEvent<{ value: string }>;

/** `checkbox-group` `bindchange` event. */
export type CheckboxGroupChangeEvent = WechatMiniprogram.CustomEvent<{ value: string[] }>;

/** `bindinput` event on a form field whose markup declares `data-field`. */
export type FieldEditEvent<Field extends string = string> = WechatMiniprogram.CustomEvent<
  { value: string },
  WechatMiniprogram.IAnyObject,
  { field: Field }
>;
