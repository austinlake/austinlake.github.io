#!/bin/bash
set -e
cargo test
cargo clippy -- -Dwarnings
rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu
cargo fmt --all
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
trunk build
git add *
git commit -m "add build artifacts"
git push
git subtree push --prefix dist origin gh-pages
rm -rf dist
git add *
git commit -m "remove build artifacts"
git push