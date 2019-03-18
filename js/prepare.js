// Game preparation

import * as dicey from "../crate/pkg/wasm_dicey";

// The canvas is expected to be square.
export const calculate_game_dimensions = (board_size, canvas_side) => {
    switch (board_size) {
    case "2x2": {
        let hex_radius = canvas_side / 4;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 2];
    };
    case "3x3": {
        let hex_radius = canvas_side / 6;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 3];
    };
    case "4x4": {
        let hex_radius = canvas_side / 8;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 4];
    };
    case "5x5": {
        let hex_radius = canvas_side / 10;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 5];
    };
    case "6x6": {
        let hex_radius = canvas_side / 12;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 6];
    };
    case "7x7": {
        let hex_radius = canvas_side / 14;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 7];
    };
    case "8x8": {
        let hex_radius = canvas_side / 16;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 8];
    };
    default: {
        let hex_radius = canvas_side / 16;
        return [dicey.Point.new(hex_radius, hex_radius), hex_radius, 2];
    };
    }
}
