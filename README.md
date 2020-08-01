# WASM-Snake

### Dev tools

In order to dev, run `make dev` from the root of the project. This will install dependencies, and start both the Rust and the Webpack watchers.

You can access the dev site at http://localhost:8080/

To run just the web part of the dev tools run `make dev-node`

### Building Rust

To build just the Rust component, to have a WASM ready to go, execute:

```
cd wasm
wasm-pack build
```

### Building web

To build the web, you will first need to have run the `Building Rust` instructions. Then run:

```
cd www
npm run build
```
