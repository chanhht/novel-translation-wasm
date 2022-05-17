# Chinese Novel to Vietnamese Translation

Chinese Novel to Vietnamese Translation

## Overview
This project use Rust to develop the core translation and compile to WebAssembly, which then able to run directly on browser. It also include a simple Vue UI application

## Local development 
```bash
# build
cargo build
# test
cargo test
```
```bash
yarn
# or
npm install
```

### Start the app in development mode (hot-code reloading, error reporting, etc.)
```bash
wasm-pack build --target web -d src/wasm
quasar dev
```


### Lint the files
```bash
yarn lint
# or
npm run lint
```

### Format the files
```bash
cargo fmt
yarn format
# or
npm run format
```

### Build the app for production
Production build
```bash
wasm-pack build --target web -d src/wasm --release
quasar build
# to deploy (required local setup)
yarn deploy
```

### Customize the configuration
See [Configuring quasar.config.js](https://v2.quasar.dev/quasar-cli-webpack/quasar-config-js).
