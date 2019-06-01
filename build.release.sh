#!/usr/bin/env bash
export PKG_CONFIG_ALLOW_CROSS=1
rustup target add x86_64-unknown-linux-musl
#rustup target add armv7-unknown-linux-musleabihf # rPi 32 bit
#rustup target add armv7-unknown-linux-gnueabihf # rPi 32 bit
#export RUSTFLAGS='-C prefer-dynamic'
export RUSTFLAGS='-C link-arg=-s' #strip debug information
#cargo deb # --target=x86_64-unknown-linux-musl
cargo deb --target=x86_64-unknown-linux-gnu
#cargo deb --target=armv7-unknown-linux-gnueabihf
ls -Ss1pq --block-size=1024 target/*/release/carusbam
ls -Ss1pq --block-size=1024 target/*/debian/*.deb
dpkg-deb -c target/*/debian/*.deb
