#!/bin/bash
set -e
cargo test
rustup component add clippy
cargo clippy -- -Dwarnings
cargo fmt --all
cargo install --locked trunk
trunk build
git subtree push --prefix dist origin gh-pages