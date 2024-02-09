import WebNesCore from "./webnes-core";

export const SCALE = 3;
const canvas = document.querySelector("canvas")!;
canvas.width = 320 * SCALE;
canvas.height = 240 * SCALE;
const ctx = canvas.getContext("bitmaprenderer", { alpha: false })!;

const core = await WebNesCore.create({
    render,
});
(window as any).core = core;

document.querySelector("button#load")!.addEventListener("click", async function selectROM() {
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