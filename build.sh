#!/usr/bin/bash

rm -rf target

cargo build --release

find target/release -maxdepth 1 -type f -executable -exec mv {} ./ \;
