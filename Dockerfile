FROM rust:alpine as builder

WORKDIR /app/src
RUN USER=root

RUN apk add pkgconfig openssl-dev libc-dev
COPY ./ ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

EXPOSE 80 443

RUN apk add openssl ca-certificates
COPY --from=builder /app/src/target/release/htwmensa /app/htwmensa

CMD ["/app/htwmensa"]