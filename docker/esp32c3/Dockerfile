# ============================================================
# ESP32-C3 development image
# ============================================================

FROM rust-embedded-base:latest

ARG ESPFLASH_VERSION=4.5.0

USER rust

RUN cargo install \
        espflash \
        --version ${ESPFLASH_VERSION} \
        --locked

RUN rustup target add riscv32imc-unknown-none-elf

WORKDIR /workspace

CMD ["bash"]
