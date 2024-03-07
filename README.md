<img src="https://codeberg.org/XDR/.profile/raw/branch/main/random/xfetch.jpg" align="right" width="300">

### xFetch

World's fastest & most simplistic fetch.

![concurrency fearless](./docs/concurrency-fearless.svg)

![binary size is miniscule](./docs/binary-size-miniscule.svg)

![simplicity by nature](./docs/simplicity-by-nature.svg)

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

![fastest fetch its true](./docs/world's-fastest-fetch-it's-true!.svg)

---

Note: `xFetch` won't show package counts for any distros that *don't* utilize *[pacman](https://wiki.archlinux.org/title/pacman)* as the package manager.

## Installation

### Arch Linux

[AUR link](https://aur.archlinux.org/packages/xfetch-bin)

```sh
yay -S xfetch-bin # for those using yay as their AUR helper
paru -S xfetch-bin # for those using paru as their AUR helper
```

### Other distros

You can [build](#build) it from source, or download an [artifact](https://github.com/XandrCopyrighted/xFetch/actions/workflows/rust.yml) from the latest run.

## Build

```sh
git clone https://github.com/XandrCopyrighted/xFetch.git
cd xFetch
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```
