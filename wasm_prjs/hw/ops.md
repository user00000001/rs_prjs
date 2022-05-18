# operations

## Install

```shell
cargo install wasm-pack
cargo install cargo-generate
# nvm for nodejs
```

## project template

```shell
cargo generate --git https://github.com/rustwasm/wasm-pack-template
# type `hw`
cd hw
wasm-pack build

npm init wasm-app www
cd www

# package.json 
# dependencies: "hw": "file:../pkg"

# index.js
# import * as wasm from "hw";
# wasm.greet() 

npm install
npm run start
```