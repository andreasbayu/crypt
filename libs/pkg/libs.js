import * as wasm from "./libs_bg.wasm";
import { __wbg_set_wasm } from "./libs_bg.js";
__wbg_set_wasm(wasm);
export * from "./libs_bg.js";
