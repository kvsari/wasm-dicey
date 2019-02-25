import * as dicey from "../crate/pkg/wasm_dicey";
import { memory } from "../crate/pkg/wasm_dicey_bg";

//import { css_colour_from_num } from "./colours.js";
import * as board from "./board.js";

const HEIGHT = 700;
const WIDTH = 700;
const DIE_COLOUR = 'white';
const DOT_COLOUR = 'black';

// Debug helping function
const drawCircle = (context, x, y, r) => {
    context.beginPath();
    context.fillStyle = "orange";
    context.arc(x, y, r, 0, 2 * Math.PI)
    context.fill();
}

// Check that a canvas (x, y) coordinate was properly translated to board (x, y).
const drawHexBoardRelativeCircle = (context, point, radius) => {
    context.beginPath();
    context.fillStyle = "silver";
    context.arc(point.x(), point.y(), radius, 0, 2 * Math.PI)
    context.fill();
}

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
const game = dicey.game_3x3_init(tl_point, 100);

// Handle clicks on the canvas
canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();
    const x = event.clientX - boundingRect.left;
    const y = event.clientY - boundingRect.top;

    // It works.
    //drawCircle(ctx, x, y, 10);

    // Offset the (x,y) coordinate since the grid is not exactly aligned with the canvas
    var board_start = game.tessellation().start_hex_center();
    var board_coord = dicey.Point.new(x - board_start.x(), y - board_start.y());

    // Check that board offset works.
    //drawHexBoardRelativeCircle(ctx, board_coord, 10);

    // Forward this coordinate to the game state and let it do its thing.
    game.select_hex_with_pixel(board_coord);

    // Finally, we draw the board. It could have changed!
    board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());
});

/*
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
*/

board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());
