FROM rust:latest

RUN cargo install cargo-fix
WORKDIR /repo

ENTRYPOINT ["cargo"]
CMD ["fix", "--allow-no-vcs", "--edition-idioms", "--broken-code"]

