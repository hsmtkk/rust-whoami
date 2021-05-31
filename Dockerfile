FROM rust:1.52.1

WORKDIR /opt

COPY src /opt/src
COPY Cargo.lock /opt/Cargo.lock
COPY Cargo.toml /opt/Cargo.toml

RUN cargo install --path .
