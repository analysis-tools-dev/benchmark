FROM rust:latest

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y clang hunspell

RUN cargo install cargo-spellcheck

WORKDIR /repo

ENTRYPOINT ["cargo"]
CMD ["spellcheck"]
