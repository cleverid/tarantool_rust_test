FROM tarantool/tarantool:2-ubuntu20.04 as build

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
COPY ./init.lua .
RUN cargo build --release

ENV LUA_CPATH=/app/target/release/lib?.so
CMD ["tarantool", "/app/init.lua"]
