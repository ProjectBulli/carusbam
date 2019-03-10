#!/usr/bin/env bash
#export RUSTFLAGS='-C prefer-dynamic'
export RUSTFLAGS='-C link-arg=-s' #strip debug information
cargo build --release
ls -Ss1pq --block-size=1024 target/release/carusbam
