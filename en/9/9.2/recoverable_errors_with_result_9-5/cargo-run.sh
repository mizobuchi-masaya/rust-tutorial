#!/bin/sh

cargo clean
rm -f ./hello.txt
cargo run

touch ./hello.txt
cargo run

exit 0
