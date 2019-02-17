import * as dicey from "../crate/pkg/wasm_dicey";
import { memory } from "../crate/pkg/wasm_dicey_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";

const HEIGHT = 600;
const WIDTH = 600;

// Quick test to see if the wasm module actually works
const greetButton = document.getElementById("greet");
greetButton.addEventListener("click", event => {
    dicey.greet()
});

// Setup our canvas
const canvas = document.getElementById("dice-board");
canvas.height = HEIGHT;
canvas.width = WIDTH;
const ctx = canvas.getContext('2d');

// Game board top left hex center point
const tl_point = dicey.Point.new(100, 100);

// Setup our game
const game = dicey.game_3x1_init(tl_point, 50);

// Draw one hex
const drawHexDetail = (detail) => {
    ctx.beginPath();

    let point = detail.point(0);
    ctx.moveTo(point.x(), point.y());

    for (var i = 1; i < 6; ++i) {
        let point = detail.point(i);
        ctx.lineTo(point.x(), point.y());
    }

    ctx.fill();
}

// Draw the entire board
const drawGameBoard = (tessellation) => {
    let length = tessellation.len();

    for (var i = 0; i < length; ++i) {
        let detail = tessellation.hex(i);
        drawHexDetail(detail);
    }
}

// Debug helping function
const drawCircle = (x, y, r) => {
    ctx.beginPath();

    ctx.arc(x, y, r, 0, 2 * Math.PI)
    ctx.stroke();
}

// Debug helping function. (Prove that a hex can be drawn).
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
    
// Our first render loop for the dummy data
const renderLoop01 = () => {
    drawPointyHex(100, 100, 100);
    requestAnimationFrame(renderLoop01);
};

// First, we'll draw a circle to help us position, align and check the hexagon.
//drawCircle(200, 200, 100);
//drawPointyHex(200, 200, 100);
//requestAnimationFrame(renderLoop01);


drawGameBoard(game.tessellation());
