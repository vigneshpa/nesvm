import { ConfigEnv, defineConfig, type PluginOption } from 'vite';
import { execSync } from 'child_process';
import wasm from "vite-plugin-wasm";

export default defineConfig( (env) => ({
    base: "./",
    build: {
        target: "ESNext"
    },
    plugins: [
        wasm(),
        wasmPackBuildPlugin(env),
    ]
}))

function wasmPackBuildPlugin(env: ConfigEnv): PluginOption {
    return {
        name: "wasm-pack-build-plugin",
        buildStart() {
            let mode = "";
            switch (env.mode) {
                case "development":
                    mode = "--dev";
                    break;
                case "production":
                    mode = "--release"
            }
            execSync(`wasm-pack build -t bundler --no-pack ${mode}`, { stdio: "inherit" });
        }
    };
}