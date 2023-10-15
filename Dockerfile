FROM rust:1.73 as builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM tarantool/tarantool:1.10.2
COPY --from=builder /app/target/release/libtarantool_rust_test.so /opt/tarantool/libtarantool_rust_test.so
COPY ./app.lua /opt/tarantool/app.lua
