interface CoreImports {
    render(fb: Uint8ClampedArray): void,
    print(text:string): void,
}
interface CoreExports extends WebAssembly.Exports {
    step(): number;
    reset(): void;
    alloc(size: number): number;
    free(ptr:number): void;
}
const decoder = new TextDecoder("UTF-8");
export async function initCore(imports: CoreImports) {
    const coreFile = new URL("../../target/wasm32-unknown-unknown/release/webnes_core.wasm", import.meta.url);
    const importObj: WebAssembly.Imports = {
        env: {
            render(p: number, n: number){
                const fb = new Uint8ClampedArray((wasm.instance.exports.memory as WebAssembly.Memory).buffer, p, n);
                console.log("Render:", fb);
                imports.render(fb);
            },
            print(p: number, n: number){
                const data = new Uint8ClampedArray((wasm.instance.exports.memory as WebAssembly.Memory).buffer, p, n);
                imports.print(decoder.decode(data));
            },
        }
    };
    const wasm = await WebAssembly.instantiateStreaming(fetch(coreFile), importObj);
    return wasm.instance.exports as CoreExports;
}