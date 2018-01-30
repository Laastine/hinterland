#!/bin/bash

set -e

mkdir -p ~/hinterland
pushd ~/hinterland

if type ~/.cargo/bin/cargo-clippy > /dev/null; then
  echo "Using cache Rust nightly and clippy"
else
  PATH=$PATH:~/rust/bin ~/rust/bin/cargo install clippy
fi
popd
