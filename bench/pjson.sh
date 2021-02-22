#!/bin/bash

set -e
cd $(dirname "${BASH_SOURCE[0]}")
wd=$(pwd)

cd ..
out=`PJSON_BENCH_FILE=bench/$1 \
    cargo test bench --release -q -- --ignored --nocapture | \
    grep "running benchmark"`
IFS=':'
read -a strarr <<< "$out"
IFS=' '
read -a strarr <<< "${strarr[2]}"
echo "${strarr[0]} GB/sec"

