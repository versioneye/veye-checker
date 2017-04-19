#!/usr/bin/env bash

#update package registry
sudo apt-get update
sudo apt-get install -y curl g++ git openssl libssl-dev pkg-config

#install rust

curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup toolchain install nightly
rustup default nightly