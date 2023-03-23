FROM rust:1.67.1

ARG PORT

WORKDIR /usr/src/app

COPY . .

ENV RUSTC_WRAPPER=bin/sccache
ENV SCCACHE_MEMCACHED=tcp://memcached:11211
ENV SCCACHE_MEMCACHED_EXPIRATION=86400

# Install diesel_cli for managing database migrations inside the container
RUN cargo install diesel_cli --no-default-features --features postgres

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

RUN cargo install --path .

EXPOSE $PORT:$PORT

CMD ["cargo", "watch", "-x", "run"]
