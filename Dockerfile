####################################
# STAGE 1: Build the binary
####################################
FROM rust:alpine AS builder

# Copy the actual source code
WORKDIR /usr/src/ssh-mcp
COPY . .

# Build the actual binary
RUN apk update && apk add --no-cache \
    openssl-dev zlib-dev musl-dev \
    openssl-libs-static zlib-static \
    && rm -rf /var/cache/apk/*
RUN cargo build --release

####################################
# STAGE 2: Create the runtime image
####################################
FROM alpine:latest

RUN apk update && apk add --no-cache \
    ca-certificates \
    libssl3 \
    && rm -rf /var/cache/apk/*

# Create a non-root user to run the application
RUN addgroup -S ssh-mcp && adduser -S -G ssh-mcp ssh-mcp
USER ssh-mcp
WORKDIR /home/ssh-mcp

# Copy the binary from the builder stage
COPY --from=builder /usr/src/ssh-mcp/target/release/ssh-mcp /usr/local/bin/ssh-mcp
COPY --from=builder /usr/src/ssh-mcp/target/release/ssh-mcp-stdio /usr/local/bin/ssh-mcp-stdio

# Expose the port the app runs on
EXPOSE 8000

# Command to run the application
CMD ["ssh-mcp"] 