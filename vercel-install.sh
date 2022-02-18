#!/bin/bash
set -x
set -e
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y 
export PATH="/vercel/.cargo/bin:$PATH" 
curl "https://github.com/rustwasm/wasm-pack/releases/download/v0.10.2/wasm-pack-v0.10.2-x86_64-unknown-linux-musl.tar.gz" -o wasm-pack.tar.gz -s 
tar xvf wasm-pack.tar.gz --wildcards --no-anchored 'wasm-pack' --strip-components=1
rm wasm-pack.tar.gz
chmod +x wasm-pack
mv wasm-pack /usr/bin
yarn install