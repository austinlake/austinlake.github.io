#!/bin/bash
set -e
cargo install rainwind
railwind -o style.css
cargo fmt --all
cargo clippy -- -Dwarnings
cargo test
trunk build
git subtree push --prefix dist origin gh-pages