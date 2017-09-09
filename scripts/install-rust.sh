#!/bin/bash

set -e

RUST_HOME=$1

if [ -d $RUST_HOME ]; then
  echo "Using cached Rust version $VERSION at $RUST_HOME"
else
  echo "Installing Rust using rustup.sh"
  curl -sSf https://static.rust-lang.org/rustup.sh | \
      sh -s -- --channel=stable --disable-sudo
fi
