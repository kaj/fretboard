# Fretboard

Draw guitar and/or mandolin fretboards with firsts, major or minor
thirds and fifts marked for given keys.
The drawing is done in the form of html for simplicity.

Here's some example output:
[for guitar](https://rasmus.krats.se/tmp/fretboard/frets.html) and
[for mandolin](https://rasmus.krats.se/tmp/fretboard/frets-mandolin.html).

## Building and installing

Fretboard is written in Rust.  To build, first
[install rust](https://www.rust-lang.org/learn/get-started#installing-rust).
Then you can build and run fretboard by checking out this project and
executing:

    cargo run -- C F#m G > frets.html

To see the results, open `frets.html` in a web browser.

If you want to install fretboard, you can compile it in release mode
(with more optimizations and less debug info) by:

    cargo build --release

You get a binary in `target/release/fretboard`, which you can copy to
`/usr/local/bin` or some other directory on your path.

There is not yet a man-page or even a `--help` option.
Sorry.
