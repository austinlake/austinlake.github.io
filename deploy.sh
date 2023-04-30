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
git subtree split --prefix dist -b gh-pages
git push -f origin gh-pages:gh-pages
git branch -D gh-pages
rm -rf dist
git add dist
git commit -m "remove build artifacts"
git push -f