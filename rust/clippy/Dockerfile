FROM rust:latest

RUN rustup component add clippy

WORKDIR /repo

CMD ["clippy"]
ENTRYPOINT ["/usr/local/cargo/bin/cargo"]
