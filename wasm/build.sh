#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
wasm-pack build --target web --out-dir ./web --no-typescript

# Feel free to build nodejs by using command options 



