FROM rust:1.57.0-slim-bullseye

COPY ./ ./

RUN cargo build --release

ENV RUST_LOG=info

CMD ["./target/release/taskmanager"]
