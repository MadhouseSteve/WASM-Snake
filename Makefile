dev: install
	make -j4 dev-rust dev-node

dev-rust:
	cd wasm ; watchexec -w src wasm-pack build

dev-node:
	cd www ; npm run dev

install:
	cd www ; npm install
