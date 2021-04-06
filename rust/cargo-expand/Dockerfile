FROM rustlang/rust:nightly-stretch-slim

RUN cargo install cargo-expand

WORKDIR /repo

ENTRYPOINT ["cargo"]
CMD ["expand"]
