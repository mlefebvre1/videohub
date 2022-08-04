# VideoHub
---

CLI and WEB application for controlling the 'Smart Videohub' SDI router from Blackmagic Design.

https://www.blackmagicdesign.com/products/smartvideohub

### How to build

For the CLI.

```
$ cargo build --release --bin cli
```

For the WEB Application.

Install WASM Target
```
$ rustup target add wasm32-unknown-unknown
```

Install Trunk
```
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

Build and package using Trunk

```
trunk build --release src/bin/web/index.html
```

### How to run


