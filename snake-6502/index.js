const canvas = document.querySelector("canvas");
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d");
ctx.lineWidth = 1;


function rng() {
    const rng =  Math.floor(Math.random() * 255);
    // console.log(rng);
    return rng;
}
function btn() {
    const btn =  Math.floor(Math.random() * 255);
    // console.log(btn);
    return btn;
}
function flog(...args) {
    console.log(...args);
}

function render(p, n) {
    const arr = new Uint8Array(wasm.instance.exports.memory.buffer, p, n);
    /** @type {ImageData} */
    const image = ctx.createImageData(32, 32);
    console.log(image);
    for(let i = 0; i < 32 ; i++) {
        for(let j = 0; j < 32 ; j++ ) {
            const k = i * 32 + j;
            const n = arr[k] > 0 ? 255 - arr[k] : 0;
            // console.log(n);

            // Red
            image.data[k*4 + 0] = n;
            // Green
            image.data[k*4 + 1] = n;
            // Blue
            image.data[k*4 + 2] = n;
            // Alpha
            image.data[k*4 + 3] = 255;
        }
    }
    ctx.putImageData(image, 0, 0);
}
const wasm = await WebAssembly.instantiateStreaming(
    fetch("../target/wasm32-unknown-unknown/release/snake6502.wasm"),
    {
        env: {
            rng,
            btn,
            render,
            flog
        }
    }
);

console.log(wasm.instance.exports);
const game = wasm.instance.exports.create();

while (true) {
    await sleep(1);
    wasm.instance.exports.step(game);
}
wasm.instance.exports.destroy(game);


function sleep(ms) {
    return new Promise(res=>setTimeout(res, ms));
}