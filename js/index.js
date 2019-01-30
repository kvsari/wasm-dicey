import * as dicey from "../crate/pkg/wasm_dicey";

const greetButton = document.getElementById("greet");

greetButton.addEventListener("click", event => {
    dicey.greet()
});
