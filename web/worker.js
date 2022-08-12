import { setup, polyfills } from "xterm-pty-wasi-polyfill";
importScripts("./xterm-pty/wasiWorkerTools.js");

onmessage = async (msg) => {
  const mod = await WebAssembly.compileStreaming(fetch('./instant-riscv.wasm'));
  const wasm = await WebAssembly.instantiate(mod, polyfills);

  let client = new TtyClient(msg.data);
  setup(wasm, client);

  wasm.exports._start();
}
