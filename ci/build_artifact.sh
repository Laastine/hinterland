#!/usr/bin/env bash

set -e

echo "Building $PACKAGE_NAME"

mkdir -p hinterland/assets
cp assets/*.png hinterland/assets
cp assets/*.json hinterland/assets
cp -r assets/maps hinterland/assets
cp -r assets/audio hinterland/assets
cp target/release/hinterland hinterland
tar zcf $PACKAGE_NAME hinterland
rm -rf hinterland
