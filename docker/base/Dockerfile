# ============================================================
# Builder
# ============================================================

FROM debian:bookworm-slim AS builder

ARG USERNAME=rust
ARG UID=1000
ARG GID=1000

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid ${GID} ${USERNAME} \
    && useradd \
        --uid ${UID} \
        --gid ${GID} \
        --create-home \
        --shell /bin/bash \
        ${USERNAME}

USER ${USERNAME}

ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal \
    && rustup component add rust-src


# ============================================================
# Development image
# ============================================================

FROM debian:bookworm-slim

ARG USERNAME=rust
ARG UID=1000
ARG GID=1000

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Common development/build dependencies.
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    ca-certificates \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Non-root development user.
RUN groupadd --gid ${GID} ${USERNAME} \
    && useradd \
        --uid ${UID} \
        --gid ${GID} \
        --create-home \
        --shell /bin/bash \
        ${USERNAME}

# Copy Rust installation from builder.
COPY --from=builder \
    /home/${USERNAME}/.cargo \
    /home/${USERNAME}/.cargo

COPY --from=builder \
    /home/${USERNAME}/.rustup \
    /home/${USERNAME}/.rustup

RUN chown -R ${UID}:${GID} \
    /home/${USERNAME}/.cargo \
    /home/${USERNAME}/.rustup

USER ${USERNAME}

WORKDIR /workspace

CMD ["bash"]
