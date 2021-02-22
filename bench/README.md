A benchmark tool that processes a bunch of JSON files from the [testfiles](../testfiles) directory.

Parsers

- [simdjson](https://github.com/simdjson/simdjson) (C++)
- [pjson](https://github.com/tidwall/pjson) (Rust)
- [serde](https://github.com/serde-rs/serde) (Rust)

Here are the results on my Macbook 2.4 GHz 8-Core Intel Core i9.

```
$ ./run.sh
apache_builds.json
   simdjson 2.04 GB/sec
   pjson    1.33 GB/sec
   serde    0.11 GB/sec
canada.json
   simdjson 0.69 GB/sec
   pjson    1.07 GB/sec
   serde    0.14 GB/sec
citm_catalog.json
   simdjson 1.86 GB/sec
   pjson    1.62 GB/sec
   serde    0.13 GB/sec
github_events.json
   simdjson 2.42 GB/sec
   pjson    1.63 GB/sec
   serde    0.13 GB/sec
gsoc-2018.json
   simdjson 0.99 GB/sec
   pjson    1.44 GB/sec
   serde    0.23 GB/sec
instruments.json
   simdjson 1.76 GB/sec
   pjson    0.98 GB/sec
   serde    0.08 GB/sec
marine_ik.json
   simdjson 0.64 GB/sec
   pjson    0.71 GB/sec
   serde    0.10 GB/sec
mesh.json
   simdjson 0.63 GB/sec
   pjson    0.63 GB/sec
   serde    0.27 GB/sec
mesh.pretty.json
   simdjson 0.91 GB/sec
   pjson    0.86 GB/sec
   serde    0.48 GB/sec
numbers.json
   simdjson 0.91 GB/sec
   pjson    0.90 GB/sec
   serde    0.44 GB/sec
random.json
   simdjson 1.42 GB/sec
   pjson    1.02 GB/sec
   serde    0.06 GB/sec
twitter.json
   simdjson 2.25 GB/sec
   pjson    1.50 GB/sec
   serde    0.11 GB/sec
twitterescaped.json
   simdjson 1.09 GB/sec
   pjson    1.08 GB/sec
   serde    0.09 GB/sec
update-center.json
   simdjson 1.76 GB/sec
   pjson    1.18 GB/sec
   serde    0.07 GB/sec
```

