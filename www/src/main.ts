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

run_app();
