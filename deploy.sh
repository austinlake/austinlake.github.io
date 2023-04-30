#!/bin/bash
set -e
cargo test
cargo clippy -- -Dwarnings
cargo fmt --all
trunk build
git subtree push --prefix dist origin gh-pages