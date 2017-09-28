#!/bin/bash

cargo build --release
mkdir -p zombie-shooter
mkdir -p zombie-shooter/assets
cp assets/*.png zombie-shooter/assets
cp assets/*.json zombie-shooter/assets
cp -r assets/maps zombie-shooter/assets
cp -r assets/audio zombie-shooter/assets
cp target/release/zombie-shooter zombie-shooter
find zombie-shooter -name '.DS_Store' -type f -delete

tar zcfv zombie-shooter.tgz zombie-shooter
rm -r zombie-shooter

echo 'done'
