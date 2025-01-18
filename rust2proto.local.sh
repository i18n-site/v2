#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

DIR_HPC=$(dirname $DIR)/hpc/coffee/rust2proto

mkdir -p gen
cd gen
rm -rf pb
cargo new --lib pb
cd ..

$DIR_HPC/build.sh

$DIR_HPC/lib/gen.js

cd gen

if ! command -v pbc &>/dev/null; then
  cargo install pbc
fi

pbc || (rm -rf pb && exit 1)

# tail -n +1 proto/*.proto
