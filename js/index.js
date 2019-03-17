import * as dicey from "../crate/pkg/wasm_dicey";
import { memory } from "../crate/pkg/wasm_dicey_bg";
import * as board from "./board.js";

const HEIGHT = 700;
const WIDTH = 700;
const DIE_COLOUR = 'white';
const DOT_COLOUR = 'black';

// Setup for our wasm.
dicey.init();

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

    // Offset the (x,y) coordinate since the grid is not exactly aligned with the canvas
    var board_start = game.tessellation().start_hex_center();
    var board_coord = dicey.Point.new(x - board_start.x(), y - board_start.y());

    // Forward this coordinate to the game state and let it do its thing.
    game.select_hex_with_pixel(board_coord);

    // Finally, we draw the board. It could have changed!
    board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());
});

board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());

