####################################################################################################
FROM clux/muslrust:stable as build

ARG SERVICE

WORKDIR /usr/src/${SERVICE}

COPY Cargo.toml Cargo.lock build.rs proto ./

# Build dependencies
RUN mkdir src/
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
RUN rm -rf src/
RUN rm -f /usr/src/${SERVICE}/target/release/${SERVICE}*
RUN rm -f /usr/src/${SERVICE}/target/release/deps/${SERVICE}*
RUN rm -f /usr/src/${SERVICE}/target/release/${SERVICE}.d

# Build app
COPY . .

RUN cargo build --release

####################################################################################################
FROM alpine:latest

ARG SERVICE

ENV APP=${SERVICE}
ENV RUST_BACKTRACE=full

RUN mkdir /app

COPY --from=build /usr/src/${SERVICE}/target/x86_64-unknown-linux-musl/release/${SERVICE} /app/${SERVICE}

ENTRYPOINT /app/${APP}
