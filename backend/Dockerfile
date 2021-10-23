FROM rust:alpine as builder

RUN apk add --no-cache libgcc g++

WORKDIR /app

COPY Cargo.toml Cargo.lock /app/
COPY src /app/src

RUN cargo build --release

##################################

FROM rust:alpine as runtime

RUN apk add --no-cache libgcc

WORKDIR /app

COPY config.toml .
COPY --from=builder /app/target/release/fizz /usr/local/bin

ENTRYPOINT ["/usr/local/bin/fizz"]
