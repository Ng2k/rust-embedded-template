# ============================================================
# ESP32 development image
# ============================================================

FROM rust-embedded-base:latest

ARG ESPUP_VERSION=0.17.1
ARG ESPFLASH_VERSION=4.5.0

USER root

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libudev-dev \
    && rm -rf /var/lib/apt/lists/*

USER rust

# ------------------------------------------------------------
# espup
# ------------------------------------------------------------

RUN cargo install \
        espup \
        --version ${ESPUP_VERSION} \
        --locked

# ------------------------------------------------------------
# Espressif Xtensa toolchain
# ------------------------------------------------------------

RUN espup install --targets esp32

# ------------------------------------------------------------
# Xtensa toolchain environment
# ------------------------------------------------------------

ENV PATH="/home/rust/.cargo/bin:/home/rust/.rustup/toolchains/esp/xtensa-esp-elf/esp-15.2.0_20250920/xtensa-esp-elf/bin:${PATH}"

# ------------------------------------------------------------
# espflash
# ------------------------------------------------------------

RUN cargo install \
        espflash \
        --version ${ESPFLASH_VERSION} \
        --locked

# ------------------------------------------------------------
# Workspace
# ------------------------------------------------------------

WORKDIR /workspace

CMD ["bash"]
