#!/bin/sh

cargo clean
rm -f ./hello.txt
cargo run

echo "John" > ./hello.txt
cargo run

exit 0
