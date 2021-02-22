#!/bin/bash

set -e
od=$(pwd)
cd $(dirname "${BASH_SOURCE[0]}")
wd=$(pwd)

serde_dir="serde"


serde_source() {
cat << EOF
use std::fs;

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let contents = fs::read_to_string(file)
            .expect("Something went wrong reading the file");
    let json = contents.as_bytes();
    let mut total = 0;
    let start = std::time::Instant::now();
    while total < 100*1024*1024 {
        let _json: serde_json::Value = serde_json::from_slice(json).unwrap();
        total += json.len();
    }
    println!(
        "{:.2} GB/sec",
        (total as f64 / start.elapsed().as_secs_f64() / 1024.0 / 1024.0 / 1024.0),
    );
}
EOF
}


if [[ "$1" == "--build" ]]; then
    if [ ! -d "$serde_dir" ]; then
        rm -rf $serde_dir-work
        mkdir -p $serde_dir-work
        cd $serde_dir-work
        cargo init
        serde_source > src/main.rs
        echo 'serde = { version = "1.0", features = ["derive"] }' >> Cargo.toml
        echo 'serde_json = "1.0"' >> Cargo.toml
        cargo build --release
        cd ..
        mv $serde_dir-work $serde_dir
    fi
    exit;
fi

cd $od
serde/target/release/serde-work "$1"
