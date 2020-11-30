# WASM Primality Test

This is a sample web application to test out the use of WASM compiled from Rust.

## Tools required

- Yarn
- Rust
- Cargo
- wasm-pack

## Building the WASM files

Source is written with Rust. Assuming that we are in the `/is-prime` directory, build for production using `wasm-pack`:

```sh
wasm-pack build --out-dir ../site/src/wasm --target web --release --no-typescript
```

This will compile, build, and output the WASM files directly within `/site/src`, which the web app can then import from.

- The target `web` is used as Rollup does not natively support bundling WASM files yet
- The `--no-typescript` is a workaround since building the website using `yarn build` with the TypeScript files will result in type errors

## Developing the website

Source is written with TypeScript and Svelte, and uses Snowpack as its development tool.

Assuming we are within the `/site` directory, install all dependencies first:

```sh
yarn
```

Develop locally with hot reload:

```sh
yarn start
```

Build for production into `/site/build`:

```sh
yarn build
```
