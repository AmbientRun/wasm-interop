import "./style.css";

import "/vite.svg";

// import client_wasm from "/wasm/client.wasm?url";
async function run_app() {
  const { default: init, start } = await import("app");

  // Fetch the wasm module
  console.info("Loaded app wasm");

  start("/wasm/client_bg.wasm");
}

run_app();
