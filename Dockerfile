FROM rust:1.45-alpine as builder
WORKDIR /usr/src/vigil
RUN apk update && apk add musl-dev openssl-dev
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/vigil /usr/local/bin/vigil
CMD ["vigil"]
