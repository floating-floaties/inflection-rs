FROM rust:latest as build

RUN USER=root cargo new --lib /sandbox
WORKDIR /sandbox

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release
RUN rm -rf ./src
RUN ls -la ./target/release/deps
RUN rm ./target/release/deps/inflection_rs*

COPY ./src ./src

RUN cargo test --release
