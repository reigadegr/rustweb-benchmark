#!/bin/bash
cd $(dirname "$0")
cargo +nightly ndk -p 34 -t arm64-v8a build --target aarch64-linux-android -r -Z trim-paths
dd if=$(dirname "$0")/target/aarch64-linux-android/release/demo-salvo of=$(dirname "$0")/demo_salvo_tokio
chmod +x demo-salvo
