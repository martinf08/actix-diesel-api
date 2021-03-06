FROM rustlang/rust:nightly

WORKDIR /app

RUN apt-get update -qq && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install diesel_cli

CMD ["tail", "-f", "/dev/null"]