# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd core/market/resolver/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv core/market/resolver/fuzz/target/x86_64-unknown-linux-gnu/release/ldap_parser /ldap_parser && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd core/market/resolver/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv core/market/resolver/fuzz/target/release/ldap_parser /ldap_parser_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /ldap_parser /ldap_parser_no_inst /
