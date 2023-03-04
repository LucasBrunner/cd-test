FROM rust:1.65 as builder
WORKDIR /usr/src/simple_web_service
COPY . .
RUN apt-get update
RUN apt-get install mingw-w64
RUN rustup target add x86_64-pc-windows-gnu
RUN cargo build --target x86_64-pc-windows-gnu --release

FROM mcr.microsoft.com/windows/nanoserver:ltsc2019
EXPOSE 8000
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/simple_web_service/target/x86_64-pc-windows-gnu/release /usr/local/bin/simple_web_service
CMD ["simple_web_service"]