FROM rust:1.72-slim-bullseye as builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential cmake && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
RUN cargo new --bin ultimate-llamagpt-mlx
WORKDIR /app/ultimate-llamagpt-mlx
COPY Cargo.toml .
RUN sed -i 's/cxx-build/cxx-build = { version = "1.0.108" }/g' Cargo.toml && \
    cargo build --release && rm src/*.rs
COPY . .
RUN touch src/main.rs && cargo build --release
FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/ultimate-llamagpt-mlx/target/release/llamagpt /usr/local/bin/
COPY assets/ /app/assets/
RUN mkdir -p /root/.config/ultimate-llamagpt-mlx
ENTRYPOINT ["llamagpt"]
CMD ["--help"]
