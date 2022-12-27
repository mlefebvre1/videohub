FROM rust:latest

RUN rustup default stable

# Install rust tools
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app/
COPY . .
