// Loading and executing the sake game
// -----------------------------------
// One could load the compiled shared library
// in a native (C/C++/Rust) project and run the game
//
// But I prefer JS and WebAssembly :)

const WASM_PATH = "../target/wasm32-unknown-unknown/release/snake_6502.wasm";
const SCALE = 20;
const canvas = new OffscreenCanvas(32, 32);
const ctx = canvas.getContext("2d");
const image = new ImageData(32, 32, {colorSpace: "srgb"});

const canvas_out = document.querySelector("canvas");
canvas_out.width = 32 * SCALE;
canvas_out.height = 32 * SCALE;
const ctx_out = canvas_out.getContext("2d");
ctx_out.scale(SCALE, SCALE);

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
    else
        return;
    e.preventDefault();
});

// WebAssembly Imported functions

function rng() {
    return Math.floor(Math.random() * 255);
}

function btn() {
    const key = lastkey;
    lastkey = 0;
    return key;
}

let rendered = false;
function render(p, n) {
    const arr = new Uint8Array(wasm.instance.exports.memory.buffer, p, n);
    constructImage(arr);
    ctx.putImageData(image, 0, 0);
    ctx_out.drawImage(canvas, 0, 0);
    rendered = true;
}

function game_reset() {
    throw new Error("Game Over");
}

// Instantiating the webassembly module

const wasm = await WebAssembly.instantiateStreaming(
    fetch(WASM_PATH),
    {
        env: {
            rng,
            btn,
            render,
            game_reset
        }
    }
);

console.log(wasm.instance.exports);

let toSkip = 0;
let t = 0;
/** @type {(timestamp: number)=>void} */
function step(timestamp) {
    const delta = timestamp - t;
    t = timestamp;
    if (toSkip > 0) {
        toSkip--;
        requestAnimationFrame(step);
        return;
    }
    let cycles = 0;
    while (!rendered) {
        cycles += wasm.instance.exports.step();
    }
    rendered = false;
    // console.log(cycles);
    toSkip = Math.floor(cycles /delta /30);
    requestAnimationFrame(step);
}
requestAnimationFrame(step);



/**
 * Construct image with the given single channel image data and scale
 * @type {(image:Uint8Array, scale: number)=>void}
 */
function constructImage(data) {
    for (let i = 0; i < 32; i++) {
        for (let j = 0; j < 32; j++) {
            const off = i * 32 + j;
            const n = data[off];
            // Red
            image.data[off * 4 + 0] = getColor(n, 0);
            // Green
            image.data[off * 4 + 1] = getColor(n, 1);
            // Blue
            image.data[off * 4 + 2] = getColor(n, 2);
            // Alpha
            image.data[off * 4 + 3] = 255;
        }
    }
}

const palette = [
    0x00, 0x00, 0x00,
    0xff, 0xff, 0xff,
    0x88, 0x00, 0x00,
    0xaa, 0xff, 0xee,
    0xcc, 0x44, 0xcc,
    0x00, 0xcc, 0x55,
    0x00, 0x00, 0xaa,
    0xee, 0xee, 0x77,
    0xdd, 0x88, 0x55,
    0x66, 0x44, 0x00,
    0xff, 0x77, 0x77,
    0x33, 0x33, 0x33,
    0x77, 0x77, 0x77,
    0xaa, 0xff, 0x66,
    0x00, 0x88, 0xff,
    0xbb, 0xbb, 0xbb,
];
/**
 * Get Color from the pallete
 * @type {(n:number, i:number)=>number}
 */
function getColor(n, i) {
    return palette[(n & 0x0f) * 3 + i];
}