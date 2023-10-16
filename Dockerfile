FROM rust:1.73 as builder

RUN USER=root cargo new --lib gant
WORKDIR /gant

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/libgant*
RUN cargo build --release

FROM tarantool/tarantool:2.10.8-ubuntu20.04
COPY --from=builder /gant/target/release/libgant.so /usr/local/lib/tarantool/gant.so
WORKDIR /gant
COPY app.lua /gant
CMD ["tarantool", "/gant/app.lua"]
