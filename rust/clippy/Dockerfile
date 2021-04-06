FROM rust:1

RUN rustup component add clippy

WORKDIR /repo

CMD ["clippy"]
ENTRYPOINT ["/usr/local/cargo/bin/cargo"]
