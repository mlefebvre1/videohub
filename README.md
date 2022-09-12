# VideoHub
---

CLI and WEB application for controlling the 'Smart Videohub' SDI router from Blackmagic Design.

https://www.blackmagicdesign.com/products/smartvideohub

### How to build
```
$ cargo build --release --workspace
```

#### For the WEB Application, build and package using trunk.


Install WASM Target
```
$ rustup target add wasm32-unknown-unknown
```

Install Trunk
```
$ cargo install --locked trunk
$ cargo install wasm-bindgen-cli
```

Build and package
```
$ trunk build --release web-sever/frontend/index.html
cargo build --release
```


### How to run

#### For the CLI.

```
$ cargo run --release -p videohub-cli -- -h
```

#### For the web-server
```
$ cargo run --release -p videohub-server
```
