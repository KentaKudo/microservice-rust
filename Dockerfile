FROM rust:1.32-slim as build

ARG SERVICE

RUN apt update
RUN apt install -y git musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/${SERVICE}

# Build dependencies
COPY Cargo.toml Cargo.toml
COPY build.rs build.rs
COPY proto proto

RUN mkdir src/

RUN echo "fn main() {}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -rf src/
RUN rm -f /usr/src/${SERVICE}/target/x86_64-unknown-linux-musl/release/${SERVICE}*
RUN rm -f /usr/src/${SERVICE}/target/x86_64-unknown-linux-musl/release/deps/${SERVICE}*
RUN rm -f /usr/src/${SERVICE}/target/x86_64-unknown-linux-musl/release/${SERVICE}.d

# Build app
COPY src/ src/

RUN RUATFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Final binary
FROM alpine:latest

ARG SERVICE

ENV APP=${SERVICE}

COPY --from=build /usr/src/${SERVICE}/target/x86_64-unknown-linux-musl/release/${SERVICE} /app/${SERVICE}

ENTRYPOINT /app/${APP}
