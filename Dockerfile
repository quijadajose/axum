# Etapa 1: Build
FROM rust:1-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: Imagen final ligera
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/axum-app /app/axum-app

EXPOSE 3000

CMD ["./axum-app"]
