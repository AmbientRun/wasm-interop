import "./style.css";

import "/vite.svg";

// import client_wasm from "/wasm/client.wasm?url";
async function run_app() {
  // https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
  const { default: init, run_loop } = await import("app");

  await init()

  // Fetch the wasm module
  console.info("Loaded app wasm");

  run_loop(1000, (i: Number) => {
    console.log(`I was called from withing WebAssembly ${i} times`)
  });
}

async function run_app2() {
  const { default: init, run_loop } = await import("app");

  await init()

  // Fetch the wasm module
  console.info("Loaded app wasm");

  run_loop(5000, (i: Number) => {
    console.log(`I am another callback and I have been called ${i} times`)
  });
}


run_app();
run_app2();
