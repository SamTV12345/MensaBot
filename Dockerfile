FROM talk.schwanzer.online/dockerhub_proxy/library/rust:alpine3.17 as builder

WORKDIR /app/src

RUN apk add pkgconfig openssl-dev libc-dev

COPY Cargo.toml .

COPY ./ ./
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

FROM alpine:latest
WORKDIR /app

RUN apk add libgcc tzdata
ENV TZ=Europe/Berlin

COPY --from=builder /app/src/target/release/htwmensa /app/htwmensa

CMD ["/app/htwmensa"]