# Stage 1: Build the application
# We use the official Rust image to compile the code.
FROM rust:slim AS builder

WORKDIR /app

# Copy the Cargo manifest and lock file to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to build dependencies first (caching optimization)
# This prevents re-downloading dependencies every time you change source code.
RUN mkdir src && \
    echo 'fn main() {println!("if you see this, the build broke")}' > src/main.rs && \
    cargo build --release

# Now copy the actual source code
COPY src src

# "Touch" the main file so Cargo knows to rebuild it, then build the release binary
RUN touch src/main.rs && cargo build --release

# Stage 2: Create the runtime image
# We use a minimal Debian image for the final container.
FROM debian:bookworm-slim

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/personal_site .

# Copy the index.html file so the server can find it
COPY assets .

# Expose port 8000 (must match the default in main.rs or the environment)
EXPOSE 8000

# Command to run the application
CMD ["./personal_site"]
