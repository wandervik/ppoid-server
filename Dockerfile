# # FROM rust:1.64
# # WORKDIR /ppoid-server
# # COPY . .
# # RUN cargo install --path .
# # # RUN cargo build --release
# # CMD ["./target/release/ppoid-server"]


# # Build Stage
# FROM rust:1.64.0 AS builder
# WORKDIR /home/ubuntu/ppoid-server
# RUN rustup target add x86_64-unknown-linux-musl

# # RUN USER=root cargo new deciduously-com
# # WORKDIR /usr/src/deciduously-com
# # COPY Cargo.toml Cargo.lock ./
# # RUN cargo build --release

# COPY . .
# RUN cargo install --target x86_64-unknown-linux-musl --path .

# # Bundle Stage
# FROM scratch
# COPY --from=builder /usr/local/cargo/bin/ppoid-server .
# CMD ["./ppoid-server"]



ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `using-sqlx`.
FROM alpine:latest
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ppoid-server \
    /usr/local/bin/
CMD /usr/local/bin/ppoid-server