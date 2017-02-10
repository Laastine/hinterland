#!/bin/bash

cargo build --release
mkdir -p hacknslash
cp assets/*.png hacknslash
cp assets/*.json hacknslash
cp -r assets/maps hacknslash
cp target/release/hacknslash hacknslash

tar zcfv hacknslash.tgz hacknslash
rm -r hacknslash

echo 'done'
