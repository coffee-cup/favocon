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
