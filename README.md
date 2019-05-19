# Favocon

Command line utility for creating
[favicons](https://en.wikipedia.org/wiki/Favicon) from a PNG image.


## Commands

Create a directory of favicons to be used on your site.

``` shell
favocon icon.png -o outdir/
```

The following favicons will be created.

``` shell
outdir/
  favicon.ico
  favicon-16x16.png
  favicon-32x32.png
```

The icon you provide must be square.

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

