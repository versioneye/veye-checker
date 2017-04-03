#!/usr/bin/env bash

#update package registry
sudo apt-get update
sudo apt-get install -y curl g++ git

#install rust

curl https://sh.rustup.rs -sSf | sh -s -- -y