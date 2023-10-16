FROM rust:1.73 as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new --lib gant
WORKDIR /gant

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/tarantool*
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM tarantool/tarantool:1.10.2
COPY --from=builder /gant/target/release/libgant.so /opt/tarantool/gant.so
COPY app.lua /opt/tarantool
CMD ["tarantool", "/opt/tarantool/app.lua"]
