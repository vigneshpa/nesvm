export const SCALE = 3;
const canvas = document.querySelector("canvas")!;
canvas.width = 320 * SCALE;
canvas.height = 240 * SCALE;
const ctx = canvas.getContext("bitmaprenderer", { alpha: false })!;

import init, { WasmNES } from "../pkg/wasmnes";

init();

class State {
    #core: WasmNES | null;
    isRunning: boolean;
    constructor() {
        this.#core = null;
        this.isRunning = false;
    }
    get core() {
        if (this.#core === null)
            throw new Error("core is null");
        return this.#core;
    }
    set core(new_core) {
        if (this.#core !== null)
            this.#core.free();
        this.#core = new_core;
    }
}
const state = new State();


export async function render(data: Uint8ClampedArray) {
    const image = new ImageData(data, 256, 240)
    const bitmap = await createImageBitmap(image, {
        resizeWidth: image.width * SCALE,
        resizeHeight: image.height * SCALE,
        resizeQuality: "pixelated",
    });
    ctx.transferFromImageBitmap(bitmap);
}

document.querySelector("button#load")!.addEventListener("click", async function selectROM() {
    if (window.showOpenFilePicker) {
        const files = await window.showOpenFilePicker({
            id: "ines-rom-select",
            multiple: false,
            types: [{
                description: "iNES ROM Files",
                accept: {
                    "application/x-ines": ".nes"
                }
            }],
            excludeAcceptAllOption: false
        });
        const rom = await files[0].getFile();
        const romData = new Uint8Array(await rom.arrayBuffer());
        state.core = new WasmNES(romData, render);
    } else {
        return await new Promise((resolve) => {
            const el = document.createElement("input");
            el.type = "file";
            el.multiple = false;
            el.addEventListener("change", async _ => {
                const buf = await el.files![0].arrayBuffer();
                const romData = new Uint8Array(buf);
                state.core = new WasmNES(romData, render);
                el.remove();
                resolve();
            });
            el.style.display = "none";
            el.accept = ".nes,application/x-ines";
            document.body.append(el);
            el.click();
        });
    }
});

// Game Loop

document.querySelector("button#start")!.addEventListener("click", function start() {
    state.isRunning = true;
    window.requestAnimationFrame(loop);
});
document.querySelector("button#reset")!.addEventListener("click", function reset() {
    state.isRunning = false;
    state.core.reset();
});

async function loop() {
    const cycles = state.core.step();
    console.log({ cycles });
    if (state.isRunning)
        window.requestAnimationFrame(loop);
}