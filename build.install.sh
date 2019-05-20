#!/usr/bin/env bash
./build.release.sh
sudo dpkg --remove carusbam
sudo dpkg -i target/x86_64-unknown-linux-musl/debian/carusbam_0.1.0_amd64.deb
sudo udevadm control --reload-rules
# sudo udevadm trigger
