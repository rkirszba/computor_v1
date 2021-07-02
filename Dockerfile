FROM rust:latest

WORKDIR /usr/src/computor_v1
COPY . .

RUN cargo install --path .
