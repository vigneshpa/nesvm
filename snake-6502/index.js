const canvas = document.querySelector("canvas");
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d");

// Key board handler
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
});

// WebAssembly Imported functions

function rng() {
    const rng = Math.floor(Math.random() * 255);
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
 * Construct image with the given single channel image data and scale
 * @type {(image:Uint8Array, scale: number)=>ImageData}
 */
function constructImage(data, scale) {
    scale = Math.floor(scale);
    /** @type {ImageData} */
    const image2 = ctx.createImageData(32 * scale, 32 * scale);
    for (let i = 0; i < 32; i++) {
        for (let j = 0; j < 32; j++) {
            const off = i * 32 + j;
            const n = data[off] > 0 ? 255 - data[off] : 0;

            const ii = i * scale;
            const jj = j * scale;
            const off2 = (ii * image2.width + jj) * 4;
            for (let k = 0; k < scale; k++) {
                const off3 = off2 + k * image2.width * 4;
                for (let l = 0; l < scale; l++) {
                    // Red
                    image2.data[off3 + l * 4 + 0] = n;
                    // Green
                    image2.data[off3 + l * 4 + 1] = n;
                    // Blue
                    image2.data[off3 + l * 4 + 2] = n;
                    // Alpha
                    image2.data[off3 + l * 4 + 3] = 255;
                }
            }
        }
    }
    return image2;
}

let rendered = false;
function render(p, n) {
    const arr = new Uint8Array(wasm.instance.exports.memory.buffer, p, n);
    const image = constructImage(arr, 10);
    ctx.putImageData(image, 0, 0);
    rendered = true;
}

// Instantiating the webassembly module
const wasm = await WebAssembly.instantiateStreaming(
    fetch("../target/wasm32-unknown-unknown/release/snake_6502.wasm"),
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

function step() {
    let cycles = 0;
    rendered = false;
    while (!rendered) {
        cycles += wasm.instance.exports.step();
    }
    requestAnimationFrame(step);
    console.log(cycles);
}
requestAnimationFrame(step);
