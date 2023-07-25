# syntax=docker/dockerfile:1
FROM alpine
RUN apk add --no-cache curl gcc
RUN curl –proto ‘=https’ –tlsv1.2 -sSf https://sh.rustup.rs | sh
ADD git@github.com:cbosoft/barbara /barbara
WORKDIR /barbara
ENV BARBARA_PORT=80
CMD cargo run --release
# UNTESTED!