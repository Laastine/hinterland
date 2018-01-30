#!/bin/bash

pushd .. && cargo build --release && popd
mkdir -p hinterland
mkdir -p hinterland/assets
cp ../assets/*.png hinterland/assets
cp ../assets/*.json hinterland/assets
cp -r ../assets/maps hinterland/assets
cp -r ../assets/audio hinterland/assets
cp ../target/release/hinterland hinterland
find hinterland -name '.DS_Store' -type f -delete

tar zcfv hinterland.tgz hinterland
rm -r hinterland

echo 'done'
