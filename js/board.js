//! Board drawing
import { css_colour_from_num } from "./colours.js";
import { drawDiceStack } from "./dice.js";

// Draw one hex
const drawHexDetail = (ctx, detail) => {
    ctx.beginPath();

    var colour = css_colour_from_num(detail.colour());

    ctx.fillStyle = colour;
    ctx.strokeStyle = 'black';

    let fpoint = detail.point(0);
    ctx.moveTo(fpoint.x(), fpoint.y());

    for (var i = 1; i < 6; ++i) {
        let point = detail.point(i);
        ctx.lineTo(point.x(), point.y());
    }
    ctx.lineTo(fpoint.x(), fpoint.y());

    ctx.fill();
    ctx.stroke();
}

// Draw the entire board
export const drawGameBoard = (ctx, die_colour, dot_colour, tessellation) => {
    let length = tessellation.len();

    for (var i = 0; i < length; ++i) {
        let detail = tessellation.hex(i);
        drawHexDetail(ctx, detail);
        drawDiceStack(
            ctx,
            die_colour,
            dot_colour,
            detail.center(),
            tessellation.radius(),
            detail.dice()
        );
    }
}

