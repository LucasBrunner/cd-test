FROM rust:1.65 as builder
WORKDIR /usr/src/simple_web_service
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
EXPOSE 8000
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/simple_web_service /usr/local/bin/simple_web_service
CMD ["simple_web_service"]