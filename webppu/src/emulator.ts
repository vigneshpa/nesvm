import { render } from "./main";

const image = new ImageData(320, 240);

function step() {
    for (let i = 0; i < image.height; i++) {
        for (let j = 0; j < image.width; j++) {
            const off = (i * image.width + j) * 4;
            image.data[off + 0] = Math.floor(Math.random() * 255);
            image.data[off + 1] = Math.floor(Math.random() * 255);
            image.data[off + 2] = Math.floor(Math.random() * 255);
            image.data[off + 3] = 255;
        }
    }
}

async function loop() {
    step();
    render(image);
    // await render(image);
    // requestAnimationFrame(loop);
}

requestAnimationFrame(loop);