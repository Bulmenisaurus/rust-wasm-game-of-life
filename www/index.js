import { render } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

let time = Date.now();

/**
 * @type {HTMLCanvasElement}
 */
const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext("2d");
const width = 2000;
const height = 2000;

canvas.width = width;
canvas.height = height;

const renderedDataPtr = render(width, height, 1);
const renderedData = new Uint8ClampedArray(
  memory.buffer,
  renderedDataPtr,
  width * height * 4
);

const imageData = ctx.createImageData(width, height);
imageData.data.set(renderedData);
ctx.putImageData(imageData, 0, 0);

alert(Date.now() - time);
