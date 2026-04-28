# Build Stage
FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN cd core/market/resolver/fuzz &&     cargo build --release &&     mv target/release/ldap_parser /ldap_parser

# Package Stage
FROM ubuntu:20.04

COPY --from=builder /ldap_parser /
