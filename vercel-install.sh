#!/bin/bash
set -x
set -e
yum install wget
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y && export PATH="/vercel/.cargo/bin:$PATH" && wget -O wasm-pack.tar.gz "https://github.com/rustwasm/wasm-pack/releases/download/v0.10.2/wasm-pack-v0.10.2-x86_64-unknown-linux-musl.tar.gz" && tar xvf wasm-pack.tar.gz && ls && yarn install