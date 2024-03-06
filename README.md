<img src="https://codeberg.org/XDR/.profile/raw/branch/main/random/xfetch.jpg" align="right" width="300">

### xFetch
A simple fetch written in Rust.

---

Note: `xFetch` only supports the *[pacman](https://wiki.archlinux.org/title/pacman)* package manager.
* [Arch User Repository](https://aur.archlinux.org/packages/xfetch-bin)
* [Codeberg Releases](https://codeberg.org/XDR/xFetch/releases)

## Build
```sh
git clone https://github.com/XandrCopyrighted/xFetch.git
cd xFetch
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```
