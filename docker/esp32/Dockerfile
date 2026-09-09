FROM rust-embedded-base:latest

ARG ESPUP_VERSION=0.17.1

USER root

# ESP32 / Xtensa development dependencies.
RUN apt-get update && apt-get install -y --no-install-recommends \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

USER rust

ENV PATH="/home/rust/.cargo/bin:/home/rust/.rustup/toolchains/esp/xtensa-esp32-elf/bin:${PATH}"

# Install espup.
RUN cargo install espup --version ${ESPUP_VERSION} --locked

# Install the Espressif Rust toolchains.
RUN espup install --targets esp32

WORKDIR /workspace

CMD ["bash"]
