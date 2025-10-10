
# Stage 1: Build the server in a Rust environment
FROM rust:1.70 AS builder
RUN cargo install matchbox_server

# Stage 2: Create the final, slim image with the compiled binary
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/matchbox_server /usr/local/bin/
EXPOSE 3536
CMD ["matchbox_server"]

