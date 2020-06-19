FROM watawuwu/rust:1.44.1 AS builder

ADD Makefile .
ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir src && \
  echo 'fn main(){}' >  src/main.rs && \
  cargo fetch

COPY . .

RUN make build CARGO_SUB_OPTIONS="--target x86_64-unknown-linux-musl --release"

FROM alpine:3.12.0

RUN apk upgrade --update-cache --available && \
  apk add openssl && \
  rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/work/target/x86_64-unknown-linux-musl/release/bump /bin/bump

ENTRYPOINT ["/bin/bump"]
