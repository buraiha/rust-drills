# syntax=docker/dockerfile:1
FROM rust:1-bookworm

ARG UID=1000
ARG GID=1000

RUN apt-get update && apt-get install -y --no-install-recommends \
    git ca-certificates pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy

ENV CARGO_HOME=/cargo
RUN mkdir -p /cargo /work

RUN groupadd -g ${GID} dev && useradd -m -u ${UID} -g ${GID} dev \
    && chown -R dev:dev /cargo /work

USER dev
WORKDIR /work
