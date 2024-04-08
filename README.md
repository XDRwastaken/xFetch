<img src="https://gitlab.com/XDRwastaken/img/-/raw/main/xFetch.jpg" align="right" width="300">

### xFetch

World's _fastest_ and simplest fetch.

![Rust Badge](https://img.shields.io/badge/made%20with%20rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Arch Badge](https://img.shields.io/badge/Only%20For%20Arch%20Linux-1793D1?logo=arch-linux&logoColor=fff&style=for-the-badge)
![Binary Badge](https://img.shields.io/badge/Binary_Size-Miniscule_(100%20kb)-7ED321?logo=hack-the-box&logoColor=fff&style=for-the-badge)
![Concurrency Badge](https://img.shields.io/badge/Concurrency-fearless-31C4f3?logo=amazon-ec2&logoColor=fff&style=for-the-badge)

## Arch User Repository

```sh
yay -S xfetch-bin # For those utilizing yay as their AUR helper.
paru -S xfetch-bin # For those utilizing yay as their AUR helper.
```

### Build

```sh
git clone https://gitlab.com/XDRwastaken/xFetch.git
cd xFetch
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```