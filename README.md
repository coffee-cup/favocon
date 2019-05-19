# Favocon

Command line utility for creating
[favicons](https://en.wikipedia.org/wiki/Favicon) from a PNG image.

![favocon-gif](https://user-images.githubusercontent.com/3044853/57985881-5dc5ae80-7a66-11e9-8a8a-fff327b83e73.gif)

## Commands

Create a directory of favicons to be used on your site.

``` shell
favocon icon.png -o outdir/
```

The icon you provide as input must be square.

## Building

- Ensure you have the [Rust toolchain
installed](https://doc.rust-lang.org/cargo/getting-started/installation.html).
- Clone this repo and cd into the directory. 
- Build the project with Cargo.

``` shell
cargo build
```

- The executable is at `target/debug/favocon/`. Copy to somewhere in your PATH.

``` shell
target/debug/favocon icon.png -o outdir
```

- Or use `cargo run`.

``` shell
cargo run -- icon.png -o outdir
```

