# syntax=docker/dockerfile:1
FROM alpine
RUN apk add --no-cache curl gcc
RUN curl –proto ‘=https’ –tlsv1.2 -sSf https://sh.rustup.rs > /rustup.sh
RUN sh /rustup.sh -y
COPY src /barbara/src
COPY Cargo.toml /barbara/Cargo.toml
WORKDIR /barbara
ENV BARBARA_PORT=80
CMD cargo run --release
# UNTESTED!