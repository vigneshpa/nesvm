const coreFile = new URL("../../target/wasm32-unknown-unknown/release/webnes_core.wasm", import.meta.url);
// const coreFile = new URL("../../target/wasm32-unknown-unknown/debug/webnes_core.wasm", import.meta.url);

interface CoreImports {
    render(fb: Uint8ClampedArray): void,
}

interface CoreWasmInstance extends WebAssembly.Instance {
    exports: {
        memory: WebAssembly.Memory,
        init(): void;
        alloc(size: number): number,
        free(ptr: number): void,
        __data_end: WebAssembly.Global,
        __heap_base: WebAssembly.Global,
        load(p: number): void,
        step(): number,
        reset(): void,
    }
}

const decoder = new TextDecoder("UTF-8");

export default class WebNesCore {
    // Init
    private imports: CoreImports;
    private wasmInstance?: CoreWasmInstance;
    static async create(imports: CoreImports) {
        const ret = new WebNesCore(imports);
        await ret.init();
        return ret;
    }
    private constructor(imports: CoreImports) {
        this.imports = imports;
    }
    private async init() {
        const core = this;
        await WebAssembly.instantiateStreaming(fetch(coreFile), {
            env: {
                render(p: number, n: number) {
                    const fb = new Uint8ClampedArray(core.wasm.exports.memory.buffer, p, n);
                    console.log("Render:", fb);
                    core.imports.render(fb);
                },
                print(p: number, n: number) {
                    const data = new Uint8Array(core.wasm.exports.memory.buffer, p, n);
                    const text = decoder.decode(data);
                    console.log("WebNesCore: ", text);
                },
                error(p: number, n: number) {
                    const data = new Uint8Array(core.wasm.exports.memory.buffer, p, n);
                    const text = "WebNesCore: " + decoder.decode(data);
                    const e = new Error(text);
                    throw e;
                }
            }
        }).then(source => {
            this.wasmInstance = source.instance as CoreWasmInstance;
            this.wasm.exports.init();
        });
    }
    private get wasm() {
        if (this.wasmInstance === undefined) {
            throw new Error("Core is not initilized");
        }
        return this.wasmInstance;
    }

    // Public API
    load(buf: ArrayBufferLike) {
        const arr = new Uint8Array(buf);
        const mem = this.alloc(buf.byteLength);
        mem.view.set(arr);
        this.wasm.exports.load(mem.pointer);
        this.free(mem);
    }
    step(): number {
        return this.wasm.exports.step();
    }
    reset(): void {
        return this.wasm.exports.reset();
    }
    alloc(_size: number) {
        const pointer = this.wasm.exports.alloc(_size);
        const wptr = new DataView(this.wasm.exports.memory.buffer, pointer, 8);
        const ptr = wptr.getUint32(0, true);
        const len = wptr.getUint32(4, true);
        const view = new Uint8Array(this.wasm.exports.memory.buffer, ptr, len);
        return { view, pointer } as DynamicMemory;
    }
    free(mem: DynamicMemory) {
        this.wasm.exports.free(mem.pointer);
    }

}

interface DynamicMemory {
    view: Uint8Array;
    pointer: number;
}