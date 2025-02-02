import init, * as wasm from "./pkg/mandelbrot.js";
await init();
window.wasm = wasm;
