import init, * as wasm from "./pkg/mandelbrot.js";

await init();
console.log("got + " + wasm.greet());

