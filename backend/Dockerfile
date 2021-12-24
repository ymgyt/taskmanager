FROM rust:1.57.0-slim-bullseye as builder

RUN USER=root cargo new --bin taskmanager
WORKDIR /taskmanager

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/taskmanager*
RUN cargo build --release


FROM debian:bullseye-slim

COPY --from=builder /taskmanager/target/release/taskmanager .

ENV RUST_LOG=info
CMD ["./taskmanager"]
