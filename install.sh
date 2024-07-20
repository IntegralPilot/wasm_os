#!/bin/bash

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
source "$HOME/.cargo/env"
rustup target install x86_64-unknown-none
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
rustup component add llvm-tools-preview
cargo install bootimage
sudo apt update
sudo apt -y install qemu-system
. "$HOME/.cargo/env"