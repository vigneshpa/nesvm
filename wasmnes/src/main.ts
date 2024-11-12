// import WasmNES from "./wasmnes";

export const SCALE = 3;
const canvas = document.querySelector("canvas")!;
canvas.width = 320 * SCALE;
canvas.height = 240 * SCALE;
const ctx = canvas.getContext("bitmaprenderer", { alpha: false })!;

import { WasmNES } from "../pkg";

class State {
    #core: WasmNES | null;
    constructor() {
        this.#core = null;
    }
    get core() {
        if (this.#core === null)
            throw new Error("core is null");
        return this.#core;
    }
    set core(new_core) {
        if (this.#core !== null)
            this.#core.drop();
        this.#core = new_core;
    }
}

const state = new State();

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
            excludeAcceptAllOption:false
        });
        const rom = await files[0].getFile();
        const romData = new Uint8Array(await rom.arrayBuffer())
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

let isRunning = false;
document.querySelector("button#start")!.addEventListener("click", function start() {
    isRunning = true;
    window.requestAnimationFrame(loop);
});
document.querySelector("button#reset")!.addEventListener("click", function reset() {
    isRunning = false;
    state.core.reset();
});


export async function render(data: Uint8Array) {
    const clampedView = new Uint8ClampedArray(data.buffer, data.byteOffset, data.byteLength);
    const image = new ImageData(clampedView, 256, 240)
    const bitmap = await createImageBitmap(image, {
        resizeWidth: image.width * SCALE,
        resizeHeight: image.height * SCALE,
        resizeQuality: "pixelated",
    });
    ctx.transferFromImageBitmap(bitmap);
}


async function loop() {
    const cycles = state.core.step();
    console.log({ cycles });
    if (isRunning)
        window.requestAnimationFrame(loop);
}