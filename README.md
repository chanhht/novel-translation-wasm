# Chinese Novel to Vietnamese Translation

Chinese Novel to Vietnamese Translation

## Overview
This project use Rust to develop the core translation and compile to WebAssembly, which then able to run directly on browser. It also include a simple Vue UI application

The project currently uses a rule-based approach inspired by an existing Windows app Quick Converter. The data is collected from various sources:
- https://hadesloki.wordpress.com/2021/08/25/share-quick-translator-data-vietphrase/comment-page-1/
- https://mephistopheles1844.wordpress.com/2021/10/08/so-sanh-cac-ban-vietphrase-cu-va-du-lieu-di-kem/

## Live demo
https://chanhht.github.io/

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
