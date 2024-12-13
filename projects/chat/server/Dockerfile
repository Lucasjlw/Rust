# Build stage for Rust application
FROM rust:1.70 as builder

# Set working directory
WORKDIR /usr/src/app

# Copy the entire project
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the Rust application
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install Nginx and required dependencies
RUN apt-get update && apt-get install -y \
    nginx \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built Rust binary
COPY --from=builder /usr/src/app/target/release/chat_server /usr/local/bin/chat-server

# Copy Nginx configuration files
COPY nginx.conf /etc/nginx/nginx.conf
COPY websocket.conf /etc/nginx/conf.d/default.conf

# Copy static files
COPY src/html/index.html /var/www/html/

# Create directory for Nginx runtime files
RUN mkdir -p /var/run/nginx

# Expose ports
EXPOSE 80

# Copy and set permissions for startup script
COPY start.sh /start.sh
RUN chmod +x /start.sh

# Start Nginx and the Rust application
CMD ["/start.sh"]