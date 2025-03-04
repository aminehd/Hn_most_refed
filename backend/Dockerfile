FROM rust:1.80 as builder

WORKDIR /app

# Copy over manifests and build dependencies to cache them
COPY Cargo.toml ./
RUN mkdir src && echo 'fn main() { println!("Dummy build!"); }' > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy over the actual source code and build
COPY . .
RUN cargo build --release

# Runtime image
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder
COPY --from=builder /app/target/release/hn_most_refed /app/
COPY .env /app/

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["/app/hn_most_refed"]