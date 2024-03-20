# Use the official Rust image as a builder
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin currency_converter
WORKDIR /currency_converter

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy the source code
COPY ./src ./src

# Build for release
RUN cargo build --release

# Final base
FROM debian:bookworm-slim  

# Install OpenSSL runtime libraries and CA certificates
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/

# Copy the build artifact from the builder stage
COPY --from=builder /currency_converter/target/release/currency_converter .

# Set the binary as the entrypoint of the container
ENTRYPOINT ["./currency_converter"]
