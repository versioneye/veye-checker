#!/usr/bin/env bash

#update package registry
apt-get update
apt-get install -y curl g++ git

#install rust

curl https://sh.rustup.rs -sSf | sh -s -- -y