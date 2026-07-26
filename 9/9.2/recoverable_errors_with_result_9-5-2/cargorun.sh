#!/bin/sh

cargo clean

touch ./hello.txt
cargo run
rm ./hello.txt
#RUST_BACKTRACE=1 cargo run
cargo run

exit 0
