import * as dicey from "../crate/pkg/wasm_dicey";
import { memory } from "../crate/pkg/wasm_dicey_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";

const HEIGHT = 600;
const WIDTH = 600;

// Declare some dummy data
//let dummy_hex_grid = dicey.HexagonGrid.default();

// Setup our canvas
const canvas = document.getElementById("dice-board");
canvas.height = HEIGHT;
canvas.width = WIDTH;
const ctx = canvas.getContext('2d');

const greetButton = document.getElementById("greet");

greetButton.addEventListener("click", event => {
    //drawGrid()
    dicey.greet()
});


// Draw a single hexagon in pointy orientation
const drawPointyHex = (x, y, r) => {
    var start_x = r;
    var start_y = 0;
    
    var point1_x = start_y * Math.sin(30) + start_x * Math.cos(30);
    var point1_y = start_y * Math.cos(30) + (-1 * start_x) * Math.sin(30);
    
    var point2_x = start_y * Math.sin(90) + start_x * Math.cos(90);
    var point2_y = start_y * Math.cos(90) + (-1 * start_x) * Math.sin(90);

    var point3_x = start_y * Math.sin(150) + start_x * Math.cos(150);
    var point3_y = start_y * Math.cos(150) + (-1 * start_x) * Math.sin(150);

    var point4_x = start_y * Math.sin(210) + start_x * Math.cos(210);
    var point4_y = start_y * Math.cos(210) + (-1 * start_x) * Math.sin(210);

    var point5_x = start_y * Math.sin(270) + start_x * Math.cos(270);
    var point5_y = start_y * Math.cos(270) + (-1 * start_x) * Math.sin(270);
    
    ctx.beginPath();
    ctx.moveTo(point1_x + x, point1_y + y);
    ctx.lineTo(point2_x + x, point2_y + y); 
    ctx.lineTo(point3_x + x, point3_y + y);
    ctx.lineTo(point4_x + x, point4_y + y);
    ctx.lineTo(point5_x + x, point5_y + y);
    ctx.fill();
}

// Our first render loop for the dummy data
const renderLoop01 = () => {

    drawPointyHex(100, 100, 100);

    requestAnimationFrame(renderLoop01);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= HEIGHT; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const renderLoop02 = () => {
    drawGrid();
    requestAnimationFrame(renderLoop02);
}

/*
// Specific hex drawing using the dummy hex grid
const drawDummyHexes = () => {
    const tilesPtr = dummy_hex_grid.tiles();
    const tiles = new Uint32Array(memory.buffer, tilesPtr, dummy_hex_grid.len());
    
    
*/

drawPointyHex(100, 100, 10);
requestAnimationFrame(renderLoop01);

