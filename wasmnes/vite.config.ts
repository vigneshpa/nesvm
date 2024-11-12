import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";

export default defineConfig({
    base: "./",
    build: {
        target: "ESNext"
    },
    plugins: [
        wasm()
    ]
})