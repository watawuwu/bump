FROM ghcr.io/watawuwu/rust:1.47.0 AS builder

ADD Makefile .
ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir src && \
  echo 'fn main(){}' >  src/main.rs && \
  cargo fetch

COPY . .

RUN make deps release-build CARGO_BUILD_TARGET="x86_64-unknown-linux-musl" CARGO_BUILD_TARGET_DIR="/usr/local/target"

FROM alpine:3.12.3

RUN apk upgrade --update-cache --available && \
  apk add openssl && \
  rm -rf /var/cache/apk/*

COPY --from=builder /usr/local/target/x86_64-unknown-linux-musl/release/bump /bin/bump

ENTRYPOINT ["/bin/bump"]
