import * as dicey from "../crate/pkg/wasm_dicey";
import { memory } from "../crate/pkg/wasm_dicey_bg";
import * as board from "./board.js";
import * as prepare from "./prepare.js";

const SIDE = 800;
const HEX_RADIUS = SIDE / (8 * 2);
const DIE_COLOUR = 'white';
const DOT_COLOUR = 'black';

// Setup for our wasm. Panic hooks n stuff.
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

// Game board top left hex center point
const tl_point = dicey.Point.new(100, 100);

// Setup our canvas
const canvas = document.getElementById("dice-board");
canvas.height = SIDE;
canvas.width = SIDE;
const ctx = canvas.getContext('2d');

// Setup our game. This is just an example one to start off.
var game = dicey.game_3x3_init(tl_point, HEX_RADIUS);

// Grab our game settings!
const playButton = document.getElementById("play");
playButton.addEventListener("click", event => {
    var board_size = document.getElementById("board-size").value;
    var player1 = document.getElementById("player1").value;
    var player2 = document.getElementById("player2").value;
    var player3 = document.getElementById("player3").value;
    var player4 = document.getElementById("player4").value;
    var turns   = document.getElementById("turns").value;
    var horizon = document.getElementById("horizon").value;

    let dimensions = prepare.calculate_game_dimensions(
        board_size, canvas.height, canvas.width, HEX_RADIUS
    );
    let tl_point = dimensions[0];
    let hex_radius = dimensions[1];
    let side = dimensions[2];

    game = dicey.start_new_game(side, tl_point, hex_radius, parseInt(turns));

    // Kick off our new game
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());

    /*
    alert(board_size + " " + player1 + " " + player2 + " " + player3 + " " + player4
          + " " + turns + " " + horizon);
    */
});

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

