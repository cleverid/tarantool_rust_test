FROM rust:1.73 as builder

RUN USER=root cargo new --lib easy
WORKDIR /easy

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/easy*
RUN cargo build --release

FROM tarantool/tarantool:2.10.8-ubuntu20.04
WORKDIR /app
COPY --from=builder /easy/target/release/libeasy.so ./easy.so
COPY init.lua .
CMD ["tarantool", "./init.lua"]
