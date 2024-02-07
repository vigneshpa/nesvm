import { initCore } from "./webnes-core";

const exports = await initCore({ 
    render,
    print(text) {
        console.log("WASM:", text);
    }
});

export const SCALE = 3;
const canvas = document.querySelector("canvas")!;
canvas.width = 320 * SCALE;
canvas.height = 240 * SCALE;
const ctx = canvas.getContext("bitmaprenderer", { alpha: false })!;

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
    const cycles = exports.step();
    // window.requestAnimationFrame(loop);
}
window.requestAnimationFrame(loop);