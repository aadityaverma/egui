

# Multi-stage build for optimized production image
FROM rust:1.70 AS builder

# Install required tools
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html ./
COPY Trunk.toml ./

# Build the application
RUN trunk build --release

# Production stage
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Expose ports
EXPOSE 80 443

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]

