#!/bin/bash
cd $(dirname "$0")
cargo +nightly ndk -p 34 -t arm64-v8a build --target aarch64-linux-android -r
dd if=$(dirname "$0")/target/aarch64-linux-android/release/demo-salvo of=$(dirname "$0")/demo-salvo
chmod +x demo-salvo
