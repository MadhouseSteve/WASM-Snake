const wasm = import("../wasm/pkg/wasm");

let game;

wasm.then((w) => {
    game = new w.Game(handleEvent, document.getElementById("board"), 20, 21, 11);

    // Hook key presses
    document.addEventListener('keydown', event => {
        let keys = ['ArrowUp', 'ArrowLeft', 'ArrowDown', 'ArrowRight'];
        if (keys.includes(event.code)) {
            event.preventDefault();
            game.handle_key_press(event.code);
        }
    });
    window.requestAnimationFrame(doTick);
});

function handleEvent(event, args) {
    switch (event) {
        case 'SET_SCORE':
            document.getElementById("score").innerText = args;
            break;
        case 'SET_LIVES':
            populateHearts(args);
            break;
        case 'DIED':
            console.log('You just died');
            break;
        case 'GAME_OVER':
            alert('game over');
            break;
    }
}

function doTick() {
    game.tick()
    window.requestAnimationFrame(doTick);
}

var heartSvgString = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
<path d="M2 0h2v1h2v1h1v2h2V2h1V1h2V0h2v1h1v1h1v8h-2v2h-2v2h-2v2H6v-2H4v-2H2v-2H0V2h1V1h1V0z" fill="#f83b3a"/>
</svg>`

const livesEl = document.getElementById('lives');

function populateHearts(int) {
    livesEl.textContent = '';
    for (let i = 0; i < int; i++) {
        const newEl = document.createElement('span');
        newEl.className = 'heartSvgHolder';
        newEl.innerHTML = heartSvgString;
        livesEl.appendChild(newEl);
    }
}


