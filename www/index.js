import * as wasm from "../wasm/pkg/wasm";

let gameLoop;
let game = new wasm.Game(document.getElementById("board"));
wasm.Game.set_board(game, false);
document.addEventListener("keyup", (event) => {
  if (
    ["ArrowLeft", "ArrowDown", "ArrowRight", "ArrowUp"].indexOf(event.code) > -1
  ) {
    if (!gameLoop) playGame();
    wasm.Game.key_press(game, event.code);
  }
});

function playGame() {
  if (gameLoop) return;

  wasm.Game.set_board(game, true);
  gameLoop = setInterval(() => {
    wasm.Game.tick(game);
    if (!wasm.Game.get_do_tick(game)) {
      clearInterval(gameLoop);
      gameLoop = null;
      gameOver();
    }
  }, 120);
}

function gameOver() {
  alert("Game over");
}
