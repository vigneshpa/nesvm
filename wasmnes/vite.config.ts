import { build, ConfigEnv, defineConfig, type PluginOption } from 'vite';
import { execSync, spawn } from 'node:child_process';
import wasm from "vite-plugin-wasm";

export default defineConfig((env) => ({
    base: "./",
    build: {
        target: "ESNext"
    },
    plugins: [
        wasm(),
        typescriptCheck(env),
        wasmPackBuildPlugin(env),
    ]
}))

function typescriptCheck(env: ConfigEnv): PluginOption {
    return {
        name: "typescript-check",
        buildStart() {
            if (env.mode == "production")
                execSync('tsc', { stdio: "inherit" })
        }
    }
}

function wasmPackBuildPlugin(env: ConfigEnv): PluginOption {
    let mode = "";
    switch (env.mode) {
        case "development":
            mode = "--dev";
            break;
        case "production":
            mode = "--release"
    }
    const wasmPackCmd = `wasm-pack build -t bundler --no-pack ${mode}`;
    return {
        name: "wasm-pack-build-plugin",
        buildStart() {
            if (env.command == "serve")
                return;
            execSync(wasmPackCmd, { stdio: "inherit" });
        },
        async configureServer(server) {
            if (env.command != "serve")
                return;

            const ignoreList = ['js', 'json', 'ts', 'html', 'css', 'gitignore'];
            const ignoreArgs: string[] = [];
            for (const type of ignoreList) {
                ignoreArgs.push('-i', '*.' + type);
            }

            const cargoWatch = spawn('cargo', ['watch', ...ignoreArgs, '-s', wasmPackCmd], {
                stdio: 'inherit',
                shell: false,
            });

            cargoWatch.on('error', (err) => {
                console.error('Cargo Watch failed: ', err);
            });

            cargoWatch.on('exit', (code) => {
                if (code !== 0) {
                    console.error(`Cargo Watch exited with code ${code}`);
                }
            });

            server.httpServer?.on('close', () => {
                cargoWatch.kill();
            });
        },
        handleHotUpdate({ server }) {
            server.ws.send({ type: "full-reload" });
            return [];
          }
    }
}