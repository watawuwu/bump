FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/bump /bin/bump

ENTRYPOINT ["/bin/bump"]
