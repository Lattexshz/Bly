# Bly
## Rusty fast cross-platform 2D graphics library
[![rust-clippy analyze](https://github.com/Lattexshz/Bly/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Lattexshz/Bly/actions/workflows/rust-clippy.yml)

# Functions to be implemented

 - [X] Window background fill
 - [ ] Rectangle Drawing

# Examples
```bash
git clone https://github.com/Lattexshz/Bly
cd Bly
cd bly
cargo run --example drawing
```

# Crates

## bly
This is the core of bly. This crate is used to do the actual drawing

## bly-ac
Provides traits for abstraction of backend crates, etc., as described below

## bly-dx2d
Bly's drawing backend for Windows (uses Direct2D internally).

## bly-cairo
Bly's drawing backend for Unix (uses Cairo internally).