# Bevy Rhythm &mdash; A Game

Based on [this tutorial](https://caballerocoll.com/blog/bevy-rhythm-game/).

## Build dependencies

The normal linker is fairly slow for the massive amount of sub-crates that Bevy
requires! So, we swap it out for the LLD linker. To do this:

- Ubuntu: `sudo apt install lld`
- Arch: `sudo pacman -S lld`
- Windows: `cargo install -f cargo-binutils` and `rustup component add llvm-tools-preview`
- MacOS: `brew install michaeleisel/zld/zld`

You'll also need a nightly build of Rust. So, using Rustup:

```bash
$ rustup toolchain install nightly
info: syncing channel updates for 'nightly-arch-platform-compiler'
# More output follows...
```

## Song Credits

The song included in this repository is _Electronic Fantasy_ by Patrick de Arteaga,
licensed under the [Creative Commons Attribution 4.0 license](https://creativecommons.org/licenses/by/4.0/). See [his website](https://patrickdearteaga.com/arcade-music/)
for this and loads more royalty-free arcade music!
