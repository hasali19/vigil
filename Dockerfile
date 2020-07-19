FROM rust:1.45-alpine as builder-rust
WORKDIR /usr/src/vigil
RUN apk update && apk add musl-dev openssl-dev
COPY . .
RUN cargo install --path .

FROM node:14-alpine as builder-node
WORKDIR /app
COPY client/package.json .
RUN npm install
COPY client .
RUN npm run build

FROM alpine
WORKDIR /app
COPY --from=builder-rust /usr/local/cargo/bin/vigil /usr/local/bin/vigil
COPY --from=builder-node /app ./client
CMD ["vigil"]
