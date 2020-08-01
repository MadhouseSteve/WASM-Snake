import("../wasm/pkg/wasm");

var heartSvgString = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
<path d="M2 0h2v1h2v1h1v2h2V2h1V1h2V0h2v1h1v1h1v8h-2v2h-2v2h-2v2H6v-2H4v-2H2v-2H0V2h1V1h1V0z" fill="#f83b3a"/>
</svg>`

const livesEl = document.getElementById('lives');

setTimeout(function() {
  populateHearts(livesEl.textContent)
}, 2000)

function populateHearts(int) {
  livesEl.textContent = '';
  for (let i = 0; i < int; i++) {
    const newEl = document.createElement('span');
    newEl.className = 'heartSvgHolder';
    newEl.innerHTML = heartSvgString;
    livesEl.appendChild(newEl);
  }
}