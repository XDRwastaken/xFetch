<img src="https://gitlab.com/XDRwastaken/img/-/raw/main/xFetch.jpg" align="right" width="300">

### xFetch

World's _fastest_ and simplest fetch.

## Arch User Repository

```sh
yay -S xfetch-bin # For those utilizing yay as their AUR helper.
paru -S xfetch-bin # For those utilizing paru as their AUR helper.
```

### Build

```sh
git clone https://gitlab.com/XDRwastaken/xFetch.git
cd xFetch
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```