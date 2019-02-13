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
    dicey.greet()
});

const drawCircle = (x, y, r) => {
    ctx.beginPath();

    ctx.arc(x, y, r, 0, 2 * Math.PI)
    ctx.stroke();
}

const drawPointyHex = (x, y, r) => {
    var point = dicey.Point.new(x, y);

    var point1 = dicey.pointy_hex_corner(point, r, 1);
    var point2 = dicey.pointy_hex_corner(point, r, 2);
    var point3 = dicey.pointy_hex_corner(point, r, 3);
    var point4 = dicey.pointy_hex_corner(point, r, 4);
    var point5 = dicey.pointy_hex_corner(point, r, 5);
    var point6 = dicey.pointy_hex_corner(point, r, 6);

    ctx.beginPath();
    ctx.moveTo(point1.x(), point1.y());
    ctx.lineTo(point2.x(), point2.y());
    ctx.lineTo(point3.x(), point3.y());
    ctx.lineTo(point4.x(), point4.y());
    ctx.lineTo(point5.x(), point5.y());
    ctx.lineTo(point6.x(), point6.y());
    ctx.fill();
}
    
/*
// Our first render loop for the dummy data
const renderLoop01 = () => {

    drawPointyHex(100, 100, 100);

    requestAnimationFrame(renderLoop01);
};
*/

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

// First, we'll draw a circle to help us position, align and check the hexagon.
drawCircle(200, 200, 100);
drawPointyHex(200, 200, 100);
//requestAnimationFrame(renderLoop01);

