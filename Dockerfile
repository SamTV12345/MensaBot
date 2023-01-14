FROM rust:alpine as builder

WORKDIR /app/src
RUN USER=root

RUN apk add pkgconfig openssl-dev libc-dev
COPY ./ ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

EXPOSE 80 443

COPY --from=builder /app/src/target/release/HTWMensa /app/HTWMensa

CMD ["/app/HTWMensa"]