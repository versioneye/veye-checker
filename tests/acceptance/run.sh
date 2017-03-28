#!/usr/bin/env bash


BIN_PATH="../../target/release"
BIN_NAME="veye_checker"

#build release binary
cargo build --release

#run acceptance tests
bash tests.sh