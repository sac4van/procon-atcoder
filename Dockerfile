FROM rust:latest

# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN cargo install cargo-atcoder

RUN mkdir -p /root/.cache/cargo-atcoder && \
    mkdir -p /root/.config/cargo-atcoder

COPY .config/cargo-atcoder.toml /root/.config/