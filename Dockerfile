FROM talk.schwanzer.online/dockerhub_proxy/library/rust:alpine3.17 as dependency-cache
USER root

RUN apk add pkgconfig openssl-dev libc-dev
WORKDIR /app/src

ADD Cargo.toml .
ADD dummy.rs ./src/main.rs
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

FROM talk.schwanzer.online/dockerhub_proxy/library/rust:alpine3.17 as builder

WORKDIR /app/src

RUN apk add pkgconfig openssl-dev libc-dev

COPY --from=dependency-cache /usr/local/cargo /usr/local/cargo
COPY --from=dependency-cache /app/src/target/ /app/src/target/
RUN rm -rf /app/src/target/release/deps/htwmensa*
RUN rm -rf /app/src/target/release/htwmensa*

COPY Cargo.toml .

COPY ./ ./
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

FROM alpine:latest
WORKDIR /app

RUN apk add libgcc tzdata
ENV TZ=Europe/Berlin

COPY --from=builder /app/src/target/release/htwmensa /app/htwmensa

CMD ["/app/htwmensa"]