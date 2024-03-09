<img src="https://codeberg.org/XDR/.profile/raw/branch/main/random/xfetch.jpg" align="right" width="300">

### xFetch

World's _fastest_ and simplest fetch.

![Rust Badge](https://img.shields.io/badge/made%20with%20rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Arch Badge](https://img.shields.io/badge/Only%20For%20Arch%20Linux-1793D1?logo=arch-linux&logoColor=fff&style=for-the-badge)
![](https://img.shields.io/badge/Binary_Size-Miniscule_(70%20kb)-7ED321?logo=hack-the-box&logoColor=fff&style=for-the-badge)
![](https://img.shields.io/badge/Concurrency-fearless-31C4f3?logo=amazon-ec2&logoColor=fff&style=for-the-badge)

## Installation

### Arch Linux

[AUR link](https://aur.archlinux.org/packages/xfetch-bin)

```sh
yay -S xfetch-bin # for those using yay as their AUR helper
paru -S xfetch-bin # for those using paru as their AUR helper
```

### From source

You can [build](#build) it from source, or download an [artifact](https://github.com/XandrCopyrighted/xFetch/actions/workflows/rust.yml) from the latest run.

## Build

```sh
git clone https://codeberg.org/XDR/xFetch.git
cd xFetch
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```