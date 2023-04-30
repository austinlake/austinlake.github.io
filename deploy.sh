#!/bin/bash
set -e
cargo test
cargo clippy -- -Dwarnings
rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu
cargo fmt --all
cargo install --locked trunk
trunk build
git subtree push --prefix dist origin gh-pages