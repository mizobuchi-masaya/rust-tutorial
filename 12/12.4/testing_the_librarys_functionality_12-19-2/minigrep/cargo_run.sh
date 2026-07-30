#!/bin/sh

cargo clean
cargo run -- the poem.txt
cargo run -- frog poem.txt
cargo run -- body poem.txt
cargo run -- monomorphization poem.txt
cargo run

exit 0
