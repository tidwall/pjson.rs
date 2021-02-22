#!/bin/bash

set -e
cd $(dirname "${BASH_SOURCE[0]}")
wd=$(pwd)

./simdjson.sh --build
./serde.sh --build

for file in ../testfiles/*.json ; do 
    if [[ "$1" == "" || "$1" == "$(basename $file)" ]]; then
        basename $file
        printf "   simdjson "
            ./simdjson.sh "../testfiles/$file"
        printf "   pjson    "
            ./pjson.sh "../testfiles/$file"
        printf "   serde    "
            ./serde.sh "../testfiles/$file"
    fi
done
