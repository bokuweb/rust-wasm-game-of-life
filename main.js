const update = Module.cwrap('update', null, ['number', 'number', 'number']);
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext('2d');
const width = window.innerWidth;
const height = window.innerHeight;
canvas.width = width;
canvas.height = height;

const image = ctx.createImageData(width, height);
const column = ~~(width);
const bufsize = ~~(height) * column;
const bufptr = Module._malloc(bufsize);

Module._memset(bufptr, 0, bufsize);
let buf = new Uint8Array(Module.HEAPU8.buffer, bufptr, bufsize);
for (let i = 0; i < buf.length; i++) {
    buf[i] = !!Math.round(Math.random());
}

const tick = () => {
    requestAnimationFrame(() => {
        console.time('t1')
        //buf = new Game(buf, ~~(bufsize / column), column).next();
        update(bufsize, buf.byteOffset, column);
        console.timeEnd('t1')
        for (let i = 0; i < bufsize; i += 1) {
            const color = buf[i] ? 0 : 0xFF;
            image.data[i * 4] = color;
            image.data[i * 4 + 1] = color;
            image.data[i * 4 + 2] = color;
            image.data[i * 4 + 3] = 0xFF;
        }
        ctx.putImageData(image, 0, 0);
        tick();
    });
};
tick();
