import wasm from "vite-plugin-wasm";
import { defineConfig } from "vite";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
    plugins: [
        wasm(),
        viteStaticCopy({
            targets: [
                {
                    src: "../target/wasm32-unknown-unknown/debug/client.wasm",
                    dest: "wasm",
                },
            ],
        }),
    ],
    server: {
        headers: {
            // Enables shared array buffers
            "Access-Control-Allow-Origin": "*",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
        },
    },
});
