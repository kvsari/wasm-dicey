// Colour translation

export const css_colour_from_num = (num) => {
    switch (num) {
    case 0: return 'black';
    case 1: return 'white';
    case 2: return 'lightblue';
    case 3: return 'blue';
    case 4: return 'darkblue';
    case 5: return 'lightgreen';
    case 6: return 'green';
    case 7: return 'darkgreen';
    case 8: return 'lightred';
    case 9: return 'red';
    case 10: return 'darkred';
    case 11: return 'lightbrown';
    case 12: return 'brown';
    case 13: return 'darkbrown';
    case 14: return 'lightyellow';
    case 15: return 'yellow';
    case 16: return 'orange';
    case 17: return 'mediumpurple';
    case 18: return 'purple';
    case 19: return 'rebeccapurple';
    default: return 'silver';
    }
}
