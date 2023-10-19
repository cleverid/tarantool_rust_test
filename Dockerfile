FROM ubuntu:20.04 as build

# Install rust from nightly
RUN apt update && \
    apt install build-essential curl -y && \
    curl https://sh.rustup.rs -sSf | bash -s -- --default-toolchain nightly -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create project and cash dependencies
RUN USER=root cargo new --lib app
WORKDIR /app
COPY ./Cargo.lock ./Cargo.toml .
RUN cargo build --release

# Copy code application and build
RUN rm src/*.rs
COPY ./src ./src
RUN cargo build --release

FROM tarantool/tarantool:2-ubuntu20.04
WORKDIR /app
COPY --from=build /app/target/release/libapp.so ./libapp.so
COPY init.lua .
ENV LUA_CPATH=/app/lib?.so
CMD ["tarantool", "./init.lua"]
