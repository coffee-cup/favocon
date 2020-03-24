# Favocon

![CI](https://github.com/coffee-cup/favocon/workflows/CI/badge.svg)
[![crates.io badge](http://meritbadge.herokuapp.com/favocon)](https://crates.io/crates/favocon)

Command line utility for creating
[favicons](https://en.wikipedia.org/wiki/Favicon) from a PNG image.

![favocon-gif](https://user-images.githubusercontent.com/3044853/57985881-5dc5ae80-7a66-11e9-8a8a-fff327b83e73.gif)

## Commands

Create a directory of favicons to be used on your site.

``` shell
favocon icon.png -o outdir/
```

The icon you provide as input must be square.

## Installation

Favocon is available through [Cargo](https://crates.io/crates/favocon).

- Install [Rust](https://www.rust-lang.org/en-US/install.html)
- `cargo install favocon`
- The `favocon` binary will be installed into `$CARGO_HOME/.bin/favocon`. This
  should be in your `PATH` already if you're using rustup.

## Building

- Ensure you have the [Rust toolchain
installed](https://doc.rust-lang.org/cargo/getting-started/installation.html).
- Clone this repo and cd into the directory. 
- Build the project with Cargo.

``` shell
cargo build
```

- Run with `cargo run`.

``` shell
cargo run -- icon.png -o outdir
```

