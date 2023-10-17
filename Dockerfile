FROM rust:1.73 as builder
WORKDIR /easy
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM tarantool/tarantool:2.10.8-ubuntu20.04
WORKDIR /app
COPY --from=builder /easy/target/release/libeasy.so ./easy.so
COPY init.lua .
CMD ["tarantool", "./init.lua"]
