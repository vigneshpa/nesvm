import { SCALE } from "./main";

export function upscalePixelated2(image: ImageData) {
    const outImage = new ImageData(image.width * SCALE, image.height * SCALE);
    for (let i = 0; i < outImage.height; i++) {
        for (let j = 0; j < outImage.width; j++) {
            const ii = Math.floor(i / SCALE);
            const jj = Math.floor(j / SCALE);

            const k = (i * outImage.width + j) * 4;
            const kk = (ii * image.width + jj) * 4;

            // outImage.data[k + 0] = image.data[kk + 0];
            // outImage.data[k + 1] = image.data[kk + 1];
            // outImage.data[k + 2] = image.data[kk + 2];
            // outImage.data[k + 3] = image.data[kk + 3];
            outImage.data.set(image.data.subarray(kk, kk + 4), k);
        }
    }
    return outImage;
}

export function upscalePixelated(image: ImageData) {
    const outImage = new ImageData(image.width * SCALE, image.height * SCALE);
    for (let i0 = 0; i0 < image.height; i0++) {
        for (let j0 = 0; j0 < image.width; j0++) {
            const i1 = i0 * SCALE;
            const j1 = j0 * SCALE;

            const k0 = (i0 * image.width + j0) * 4;
            const k1 = (i1 * outImage.width + j1) * 4;

            for (let i2 = 0; i2 < SCALE; i2++) {
                for (let j2 = 0; j2 < SCALE; j2++) {
                    const k2 = k1 + (i2 * outImage.width + j2) * 4;
                    outImage.data[k2 + 0] = image.data[k0 + 0];
                    outImage.data[k2 + 1] = image.data[k0 + 1];
                    outImage.data[k2 + 2] = image.data[k0 + 2];
                    outImage.data[k2 + 3] = image.data[k0 + 3];
                    // outImage.data.set(image.data.subarray(k0, k0 + 4), k2);
                }
            }
        }
    }
    return outImage;
}