FROM rust-embedded-base:latest

USER rust

# ESP32-C3 development dependencies.
RUN cargo install cargo-espflash --version 4.5.0 --locked

# RISC-V target used by ESP32-C3.
RUN rustup target add riscv32imc-unknown-none-elf

WORKDIR /workspace

CMD ["bash"]
