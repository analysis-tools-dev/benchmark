FROM rust:latest

RUN cargo install cargo-spellcheck

WORKDIR /repo

ENTRYPOINT ["cargo"]
CMD ["spellcheck"]
