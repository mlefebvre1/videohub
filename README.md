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
$ trunk build --release web/index.html
```


### How to run

#### For the CLI.

```
$ cargo run --release --manifest-path "cli/Cargo.toml" -- <args>
```

#### For the WEB Application (not ready)
```
$ trunk serve --release web/index.html
```