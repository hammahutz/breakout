#!/bin/sh
cargo build -r
cp ./assets ./target/release/ -rvu
