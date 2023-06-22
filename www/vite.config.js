import wasm from "vite-plugin-wasm";
import { defineConfig } from "vite";

export default defineConfig({
    plugins: [wasm()],
    server: {
        headers: {
            // Enables shared array buffers
            "Access-Control-Allow-Origin": "*",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
        },
    },
});
