#!/bin/sh

cargo clean
cargo run -- the poem.txt
cargo run -- frog poem.txt
cargo run -- body poem.txt
cargo run -- monomorphization poem.txt
cargo run
cargo run -- to poem.txt
IGNORE_CASE=1 cargo run -- to poem.txt
rm -f ./output.txt.ERR ./output.txt.OK
cargo run > ./output.txt.ERR
cargo run -- to poem.txt > ./output.txt.OK

exit 0
