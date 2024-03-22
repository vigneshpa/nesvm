import WasmNES from "./wasmnes";

export const SCALE = 3;
const canvas = document.querySelector("canvas")!;
canvas.width = 320 * SCALE;
canvas.height = 240 * SCALE;
const ctx = canvas.getContext("bitmaprenderer", { alpha: false })!;

const core = await WasmNES.create({ render });
(window as any).core = core;

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
        core.load(await rom.arrayBuffer());
    } else {
        return await new Promise((resolve) => {
            const el = document.createElement("input");
            el.type = "file";
            el.multiple = false;
            el.addEventListener("change", async _ => {
                const buf = await el.files![0].arrayBuffer();
                core.load(buf);
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
    core.reset();
});


export async function render(data: Uint8ClampedArray) {
    const image = new ImageData(data, 256, 240)
    const bitmap = await createImageBitmap(image, {
        resizeWidth: image.width * SCALE,
        resizeHeight: image.height * SCALE,
        resizeQuality: "pixelated",
    });
    ctx.transferFromImageBitmap(bitmap);
}


async function loop() {
    const cycles = core.step();
    console.log({ cycles });
    if (isRunning)
        window.requestAnimationFrame(loop);
}