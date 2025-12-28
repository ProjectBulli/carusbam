#!/usr/bin/env bash
cargo deb --target=x86_64-unknown-linux-gnu # --target=aarch64-unknown-linux-gnu
ls -Ss1pq --block-size=1024 target/*/release/carusbam
ls -Ss1pq --block-size=1024 target/debian/*.deb
dpkg-deb -c target/debian/*.deb
