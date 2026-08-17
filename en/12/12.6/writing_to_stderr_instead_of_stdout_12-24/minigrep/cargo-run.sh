#!/bin/sh

rm -f ./output.txt ./output2.txt
cargo clean
cargo run > output.txt
cargo run -- to poem.txt > output2.txt

exit 0
