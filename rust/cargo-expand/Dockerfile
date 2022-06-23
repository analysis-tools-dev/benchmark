FROM rust:latest

RUN cargo install cargo-expand

WORKDIR /repo

ENTRYPOINT ["cargo"]
CMD ["expand"]
