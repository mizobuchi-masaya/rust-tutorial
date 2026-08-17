#!/bin/sh

cargo clean
cargo run -- frog poem.txt
cargo run -- body poem.txt
cargo run -- monomorphization  poem.txt

exit 0
