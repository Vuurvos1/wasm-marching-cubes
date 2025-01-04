# Marching Cubes Meta Balls

A simple implementation of the marching cubes algorithm with meta balls in Rust and WebAssembly.

> [!NOTE]  
> Currently the performance WASM performance seems to be about 1.6x faster than the THREE.js implementation but there might be more room for optimization.

## Developing

Once you've created a project and installed dependencies with `pnpm install`, start a development server:

```bash
pnpm dev

# or start the server and open the app in a new browser tab
pnpm dev -- --open
```

## Building

To create a production version of your app:

```bash
pnpm build
```

### Building the wasm module

To build the wasm module, you need to install `wasm-bindgen-cli`:

```bash
cargo install wasm-bindgen-cli
```

Then you can build the wasm module with:

```bash
pnpm build:wasm
```

You can preview the production build with `pnpm preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
