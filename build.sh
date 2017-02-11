#!/bin/bash

cargo build --release
mkdir -p hacknslash
mkdir -p hacknslash/assets
cp assets/*.png hacknslash/assets
cp assets/*.json hacknslash/assets
cp -r assets/maps hacknslash/assets
cp -r assets/audio hacknslash/assets
cp target/release/hacknslash hacknslash

tar zcfv hacknslash.tgz hacknslash
rm -r hacknslash

echo 'done'
