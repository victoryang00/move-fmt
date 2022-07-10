Move (IR) Formatter
==============

[![Rust](https://github.com/victoryang00/move-fmt/actions/workflows/rust.yml/badge.svg)](https://github.com/victoryang00/move-fmt/actions/workflows/rust.yml) 

A formatter written in rust pest with detailed grammar.

## How to use
```bash
cargo run -- [your file path]
```

## Default Configs

- indent: usize: 4(default)
- space between columns: usize: 0(default)
- space from equal: usize: 1(default)