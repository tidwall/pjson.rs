# pjson-rs

[![license](https://img.shields.io/crates/l/pjson.svg?)](LICENSE)
[![crates.io](https://img.shields.io/crates/d/pjson.svg)](https://crates.io/crates/pjson)
[![version](https://img.shields.io/crates/v/pjson.svg)](https://crates.io/crates/pjson/)
[![documentation](https://docs.rs/pjson/badge.svg)](https://docs.rs/pjson/)

A JSON stream parser for Rust.  

This is a port of the [pjson](https://github.com/tidwall/pjson) Go library.

## Example

Print all string values from a JSON document.

```rust
fn main() {

    let json = br#"
    {
      "name": {"first": "Tom", "last": "Anderson"},
      "age":37,
      "children": ["Sara","Alex","Jack"],
      "fav.movie": "Deer Hunter",
      "friends": [
        {"first": "Dale", "last": "Murphy", "age": 44, "nets": ["ig", "fb", "tw"]},
        {"first": "Roger", "last": "Craig", "age": 68, "nets": ["fb", "tw"]},
        {"first": "Jane", "last": "Murphy", "age": 47, "nets": ["ig", "tw"]}
      ]
    }
    "#;

    pjson::parse(json, 0, |start: usize, end: usize, info: usize) i64 {
        if info&(pjson::STRING|pjson::VALUE) == pjson::STRING|pjson::VALUE {
            let el = String::from_utf8(json[start..end].to_vec()).unwrap();
            println!("{}", el);
        }
        1
    });

}

// output:
// "Tom"
// "Anderson"
// "Sara"
// "Alex"
// "Jack"
// "Deer Hunter"
// "Dale"
// "Murphy"
// "ig"
// "fb"
// "tw"
// "Roger"
// "Craig"
// "fb"
// "tw"
// "Jane"
// "Murphy"
// "ig"
// "tw"
```

