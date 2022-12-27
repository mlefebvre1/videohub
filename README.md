# VideoHub
---

CLI and WEB application for controlling the 'Smart Videohub' SDI router from Blackmagic Design.

https://www.blackmagicdesign.com/products/smartvideohub

### How to use (without docker)
#### Get the latest stable rust version
```
$ rustup default stable
$ rustup update
```
#### (Optional) Install rust tools if you are using the web-server
```
$ rustup target add wasm32-unknown-unknown
$ cargo install trunk wasm-bindgen-cli
```
#### Build the workspace
```
$ cargo build
```

#### Build the frontend (if using the web-server)
```
$ trunk build web-server/frontend/index.html
```

#### Run the CLI
```
$ cargo run -p videohub-cli -- -h
```
#### Run the web-server
```
cargo run -p videohub-server
```
---
### How to use with docker (web-server only)
#### Build the base docker image
```
$ docker-compose build base
```
#### Start the service for router
```
$ docker-compose up web-router
```
