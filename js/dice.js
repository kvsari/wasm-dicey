// Dice drawing. It's a bit involved due to non-implementation of `IntoWasmAbi` for
// `Point` types in the rust wasm.
import * as dicey from "../crate/pkg/wasm_dicey";

const drawDot = (ctx, dot_colour, dot) => {    
    ctx.beginPath();
    ctx.fillStyle = dot_colour;
    ctx.arc(dot.x(), dot.y(), dot.radius(), 0, 2 * Math.PI);
    ctx.fill();
}


// Draw dice on the hex
const drawHexDice = (
    ctx, die_colour, dot_colour, center, radius, position, dots
) => {
    var dice = dicey.DiceTemplate.new(center, radius, position);

    // Draw the die square
    ctx.beginPath();

    ctx.fillStyle = die_colour;
    ctx.strokeStyle = dot_colour;
    
    ctx.rect(dice.x(), dice.y(), dice.width(), dice.height());
    
    ctx.fill();
    ctx.stroke();

    // Then add the dots. We do this beginner style with lots of `if` statements.
    if (dots == 1) {
        var dot = dice.dot(dicey.Position.Center);
        drawDot(ctx, dot_colour, dot);
    }

    if (dots == 2) {
        var dot1 = dice.dot(dicey.Position.TopLeft);
        var dot2 = dice.dot(dicey.Position.BottomRight);
        drawDot(ctx, dot_colour, dot1);
        drawDot(ctx, dot_colour, dot2);
    }

    if (dots == 3) {
        var dot1 = dice.dot(dicey.Position.TopLeft);
        var dot2 = dice.dot(dicey.Position.Center);
        var dot3 = dice.dot(dicey.Position.BottomRight);
        drawDot(ctx, dot_colour, dot1);
        drawDot(ctx, dot_colour, dot2);
        drawDot(ctx, dot_colour, dot3);
    }

    if (dots == 4) {
        var dot1 = dice.dot(dicey.Position.TopLeft);
        var dot2 = dice.dot(dicey.Position.TopRight);
        var dot3 = dice.dot(dicey.Position.BottomLeft);
        var dot4 = dice.dot(dicey.Position.BottomRight);
        drawDot(ctx, dot_colour, dot1);
        drawDot(ctx, dot_colour, dot2);
        drawDot(ctx, dot_colour, dot3);
        drawDot(ctx, dot_colour, dot4);
    }

    if (dots == 5) {
        var dot1 = dice.dot(dicey.Position.TopLeft);
        var dot2 = dice.dot(dicey.Position.TopRight);
        var dot3 = dice.dot(dicey.Position.BottomLeft);
        var dot4 = dice.dot(dicey.Position.BottomRight);
        var dot5 = dice.dot(dicey.Position.Center);
        drawDot(ctx, dot_colour, dot1);
        drawDot(ctx, dot_colour, dot2);
        drawDot(ctx, dot_colour, dot3);
        drawDot(ctx, dot_colour, dot4);
        drawDot(ctx, dot_colour, dot5);
    }
}

export const drawDiceStack = (ctx, die_colour, dot_colour, center, radius, dice) => {
    if (dice > 0) {
        var position = dicey.Position.TopLeft;
        drawHexDice(ctx, die_colour, dot_colour, center, radius, position, 1);
    }

    if (dice > 1) {
        var position = dicey.Position.TopRight;
        drawHexDice(ctx, die_colour, dot_colour, center, radius, position, 2);
    }

    if (dice > 2) {
        var position = dicey.Position.BottomLeft;
        drawHexDice(ctx, die_colour, dot_colour, center, radius, position, 3);
    }

    if (dice > 3) {
        var position = dicey.Position.BottomRight;
        drawHexDice(ctx, die_colour, dot_colour, center, radius, position, 4);
    }

    if (dice > 4) {
        var position = dicey.Position.Center;
        drawHexDice(ctx, die_colour, dot_colour, center, radius, position, 5);
    }
}
