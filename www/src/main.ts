import "./style.css";

import "/vite.svg";

// import client_wasm from "/wasm/client.wasm?url";
async function run_app() {
  const { default: init, run_loop } = await import("app");

  // Fetch the wasm module
  console.info("Loaded app wasm");

  run_loop(1000, (i: Number) => {
    console.log(`I was called from withing WebAssembly ${i} times`)
  });
}

async function run_app2() {
  const { default: init, run_loop } = await import("app");

  // Fetch the wasm module
  console.info("Loaded app wasm");

  run_loop(5000, (i: Number) => {
    console.log(`I am another callback and I have been called ${i} times`)
  });
}


run_app();
run_app2();
