FROM rust:latest as build-env
#RUN apk update
WORKDIR /app
COPY ./src/ ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build