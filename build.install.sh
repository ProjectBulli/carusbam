#!/usr/bin/env bash
./build.release.sh
sudo apt reinstall ./target/debian/*.deb
sudo udevadm control --reload-rules
