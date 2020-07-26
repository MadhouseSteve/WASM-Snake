import * as wasm from "../wasm/pkg/wasm";

let game = new wasm.Game(document.getElementById("board"));
wasm.Game.set_board(game);

setInterval(() => {
  wasm.Game.tick(game);
}, 15);

document.addEventListener("keyup", (event) =>
  wasm.Game.key_press(game, event.code)
);
