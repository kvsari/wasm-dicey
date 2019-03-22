import * as dicey from "../crate/pkg/wasm_dicey";
import { memory } from "../crate/pkg/wasm_dicey_bg";
import * as board from "./board.js";
import * as prepare from "./prepare.js";
//import * as advance from "./advance.js";

const SIDE = 800;
const HEX_RADIUS = SIDE / 6;
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
const tl_point = dicey.Point.new(HEX_RADIUS, HEX_RADIUS);

// Setup our canvas
const canvas = document.getElementById("dice-board");
canvas.height = SIDE;
canvas.width = SIDE;
const ctx = canvas.getContext('2d');

// Setup our game. This is just an example one to start off.
var game = dicey.game_3x3_init(tl_point, HEX_RADIUS);

const display_player = (num, moves, captured, ai) => {
    if (ai) {
        document.getElementById("play-status").innerText = "AI Player" + num +
            ". Moves Left: " + moves + ". Captured Dice: " + captured;
    } else {
        document.getElementById("play-status").innerText = "Current: Player" + num +
            ". Moves Left: " + moves + ". Captured Dice: " + captured;
    }
}

const add_battle_log = (entry) => {
    let br = document.createElement("br");
    let p = document.createElement("p");
    let content = document.createTextNode(entry);
    p.appendChild(content);
    
    let heading = document.getElementById("battle-heading");
    let section = heading.parentNode;
    section.insertBefore(content, heading.nextSibling);
    section.insertBefore(br, heading.nextSibling);
}

const add_battle_log_items = (items) => {
    let entries = items.split("\n");
    for (var i in entries) {
        add_battle_log(entries[i]);
    }
}

const play_on = () => {
    while (game.advance()) {
        let log_items = game.state_log();
        let player_id = game.current_player_id();
        let player_moves_left = game.current_player_moves_left();
        let captured_dice = game.current_player_dice_captured();
        let is_ai = game.current_player_ai();
        display_player(player_id, player_moves_left, captured_dice, is_ai);
        board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());
        add_battle_log_items(log_items);
        setTimeout(function() { 1 + 1; }, 1000);
    }
}

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

    game = dicey.start_new_game(
        side,
        tl_point,
        hex_radius,
        parseInt(turns),
        prepare.player_option_to_code(player1),
        prepare.player_option_to_code(player2),
        prepare.player_option_to_code(player3),
        prepare.player_option_to_code(player4),
        parseInt(horizon)
    );

    let player_id = game.current_player_id();
    let player_moves_left = game.current_player_moves_left();
    let captured_dice = game.current_player_dice_captured();
    let is_ai = game.current_player_ai();
    display_player(player_id, player_moves_left, captured_dice, is_ai);

    // Kick off our new game
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());
    add_battle_log("New game starting!");
    play_on();
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
    if (game.select_hex_with_pixel(board_coord)) {
        let log_items = game.state_log();
        add_battle_log_items(log_items);
    }
    
    // Update the play-status with any changes.
    let player_id = game.current_player_id();
    let player_moves_left = game.current_player_moves_left();
    let captured_dice = game.current_player_dice_captured();
    display_player(player_id, player_moves_left, captured_dice);
    
    // Finally, we draw the board. It could have changed!
    board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());
    play_on();
});

board.drawGameBoard(ctx, DIE_COLOUR, DOT_COLOUR, game.tessellation());

