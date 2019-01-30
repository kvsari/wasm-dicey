# WASM DICEY
A wasm frontend for `dicey-dice`.

## Prerequisites
1. npm (recommended to use [nvm](https://github.com/creationix/nvm))
2. [rust](https://www.rust-lang.org/)
3. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## How to build
Make sure that the wasm modules have been built first. Inside the `./crate` dir;
```console
wasm-pack build
```
This will create a `pkg/` directory containing all our wasm stuff.

Then, in the project root, run
```console
npm install 
```
to get all our dependencies.

## Run development server
To run locally using the development server, after [building](#how-to-build), run
```console
npm run start
```

