#!/bin/sh

cargo clean
cargo run -- to poem.txt
IGNORE_CASE=1 cargo run -- to poem.txt

exit 0
