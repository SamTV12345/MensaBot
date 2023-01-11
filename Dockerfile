FROM rustlang/rust:nightly as builder

WORKDIR /app/src
RUN USER=root

COPY ./ ./
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 80 443

COPY --from=builder /app/src/target/release/HTWMensa /app/HTWMensa

CMD ["/app/HTWMensa"]