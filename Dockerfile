# 1: Build the executable for npuzzle
FROM rust:latest as builder
WORKDIR /usr/src

# Prepare container
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# Download and compile Rust dependencies
RUN USER=root cargo new npuzzle
WORKDIR /usr/src/npuzzle
COPY Cargo.toml Cargo.lock ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Build the executable using the source code
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 2: Copy the exe and extra files if needed to an empty Docker image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/npuzzle .
COPY maps .
USER 1000
ENTRYPOINT ["./npuzzle"]