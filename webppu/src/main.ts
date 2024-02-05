import "./emulator";
// import { upscalePixelated } from "./upscaling";

export const SCALE = 3;
const canvas = document.querySelector("canvas")!;
canvas.width = 320 * SCALE;
canvas.height = 240 * SCALE;
const ctx = canvas.getContext("bitmaprenderer", { alpha: false })!;

export async function render(image: ImageData) {
    const bitmap = await createImageBitmap(image, {
        resizeWidth: image.width * SCALE,
        resizeHeight: image.height * SCALE,
        resizeQuality: "pixelated",
    });
    ctx.transferFromImageBitmap(bitmap);
}