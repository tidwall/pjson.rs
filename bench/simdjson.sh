#!/bin/bash

set -e
od=$(pwd)
cd $(dirname "${BASH_SOURCE[0]}")
wd=$(pwd)

if [[ ! -f simdjson.vers ]]; then
    curl https://api.github.com/repos/simdjson/simdjson/releases/latest -s | \
        jq .tag_name -r > simdjson.vers
fi
simdjson_latest=$(cat simdjson.vers)
simdjson_dir="simdjson-$simdjson_latest"
simdjson_dir_abs=$wd/$simdjson_dir

simdjson_source() {
cat << EOF
#include "simdjson.h"
using namespace simdjson;
int main(int argc, char **argv) {
    FILE *f = fopen(argv[1], "rb");
    assert(f);
    fseek(f, 0, SEEK_END);
    int size = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *tjson = (char *)malloc(size+64);
    fread(tjson, 1, size, f);
    tjson[size] = 0;
    fclose(f);

    uint64_t bytes = 0;
    clock_t begin = clock();
    simdjson::dom::parser parser;
    while (bytes < 100*1024*1024) {
        parser.parse(tjson, size);
        bytes += size;
    }
    clock_t end = clock();
    double elapsed_secs = (double)(end - begin) / CLOCKS_PER_SEC;
    double bytes_sec = (double)bytes/elapsed_secs;
    printf("%.2f GB/sec\n",
        bytes_sec/1024/1024/1024
    );
}
EOF
}

if [[ "$1" == "--build" ]]; then
    if [ ! -d "$simdjson_dir" ]; then
        rm -rf $simdjson_dir-work
        mkdir -p $simdjson_dir-work
        cd $simdjson_dir-work
        wget -c https://github.com/simdjson/simdjson/archive/$simdjson_latest.tar.gz
        tar -xf $simdjson_latest.tar.gz
        mv simdjson-*/* .
        cd ..
        mv $simdjson_dir-work $simdjson_dir
    fi

    if [ ! -f "$simdjson_dir/a.out" ]; then
        cd $simdjson_dir
        simdjson_source > simdjson.cpp
        c++ -Iinclude -Isrc -std=c++11 -O3 simdjson.cpp src/simdjson.cpp
        cd ..
    fi
    exit;
fi

cd $od
$simdjson_dir_abs/a.out "$1"
