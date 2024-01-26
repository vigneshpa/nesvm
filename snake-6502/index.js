const canvas = document.querySelector("canvas");
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d");
ctx.lineWidth = 1;

let lastkey = 0;
document.body.addEventListener("keydown", e => {
    lastkey = e.key.charCodeAt(0);
    if (e.keyCode === 37)
        lastkey = "a".charCodeAt(0);
    else if (e.keyCode === 38)
        lastkey = "w".charCodeAt(0);
    else if (e.keyCode === 39)
        lastkey = "d".charCodeAt(0);
    else if (e.keyCode === 40)
        lastkey = "s".charCodeAt(0);
    // console.log(e);
});
function rng() {
    const rng = Math.floor(Math.random() * 255);
    // console.log(rng);
    return rng;
}
function btn() {
    const key = lastkey;
    lastkey = 0;
    return key;
}
function reset() {
    throw new Error("Game Over");
}

/**
 * @type {(image:ImageData, scale: number)=>ImageData}
 */
function scaleImage(image, scale) {
    scale = Math.floor(scale);
    /** @type {ImageData} */
    const image2 = ctx.createImageData(image.width * scale, image.height * scale);
    for (let i = 0; i < image.width; i++) {
        for (let j = 0; j < image.height; j++) {
            const off = (i * image.width + j) * 4;
            const ii = i * scale;
            const jj = j * scale;
            const off2 = (ii * image2.width + jj) * 4;
            for(let k = 0; k < scale; k++) {
                const off3 = off2 + k * image2.width * 4;
                for(let l = 0; l < scale; l++) {
                    image2.data[off3 + l * 4 + 0] = image.data[off + 0];
                    image2.data[off3 + l * 4 + 1] = image.data[off + 1];
                    image2.data[off3 + l * 4 + 2] = image.data[off + 2];
                    image2.data[off3 + l * 4 + 3] = image.data[off + 3];
                }
            }
        }
    }
    return image2;
}

let rendered = false;
function render(p, n) {
    const arr = new Uint8Array(wasm.instance.exports.memory.buffer, p, n);
    /** @type {ImageData} */
    const image = ctx.createImageData(32, 32);
    // console.log(image);
    for (let i = 0; i < 32; i++) {
        for (let j = 0; j < 32; j++) {
            const k = i * 32 + j;
            const n = arr[k] > 0 ? 255 : 0;
            // console.log(n);

            // Red
            image.data[k * 4 + 0] = n;
            // Green
            image.data[k * 4 + 1] = n;
            // Blue
            image.data[k * 4 + 2] = n;
            // Alpha
            image.data[k * 4 + 3] = 255;
        }
    }
    // ctx.putImageData(image, 0, 0);
    ctx.putImageData(scaleImage(image, 10), 0, 0);
    rendered = true;
}
const wasm = await WebAssembly.instantiateStreaming(
    fetch("../target/wasm32-unknown-unknown/release/snake.wasm"),
    {
        env: {
            rng,
            btn,
            render,
            reset
        }
    }
);

console.log(wasm.instance.exports);
const game = wasm.instance.exports.create();

function step() {
    try {
        while (!rendered) {
            wasm.instance.exports.step(game);
        }
        rendered = false;
        requestAnimationFrame(step);
    }catch(e){
        wasm.instance.exports.destroy(game);
        throw e;
    }
}
requestAnimationFrame(step);


// function sleep(ms) {
//     return new Promise(res=>setTimeout(res, ms));
// }