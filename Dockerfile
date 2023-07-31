FROM rust:alpine as builder
RUN apk add --no-cache musl-dev mold clang16
WORKDIR /app
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release && rm -rf src
ADD . ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/hello-world .
CMD ["/app/hello-world"]
