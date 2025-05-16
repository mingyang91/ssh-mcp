####################################
# STAGE 1: Build the binary
####################################
FROM rust AS builder

# Copy the actual source code
WORKDIR /usr/src/ssh-mcp
COPY . .

# Build the actual binary
RUN cargo build --release

####################################
# STAGE 2: Create the runtime image
####################################
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN useradd -m ssh-mcp
USER ssh-mcp
WORKDIR /home/ssh-mcp

# Copy the binary from the builder stage
COPY --from=builder /usr/src/ssh-mcp/target/release/ssh-mcp /usr/local/bin/ssh-mcp
COPY --from=builder /usr/src/ssh-mcp/target/release/ssh-mcp-stdio /usr/local/bin/ssh-mcp-stdio

# Expose the port the app runs on
EXPOSE 8000

# Command to run the application
CMD ["ssh-mcp"] 