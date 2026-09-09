# Firmware Build Guide

This document describes how to build the firmware targets provided by the template.

The repository currently supports:

* ESP32
* ESP32-C3

Each firmware target is an independent Cargo project with its own toolchain and target configuration.

## Prerequisites

For local builds, install:

* Rust and `rustup`
* `espup` for ESP32 development
* `espflash`
* Git

The ESP32 requires the Espressif Rust toolchain. ESP32-C3 uses the standard Rust toolchain.

For Docker-based builds, see [DOCKER.md](DOCKER.md).

## Firmware Projects

Firmware projects are located under:

```text
firmware/
├── esp32/
└── esp32c3/
```

Each firmware contains its own:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
build.rs
.cargo/config.toml
src/bin/main.rs
```

The projects are intentionally independent because they target different architectures and toolchains.

## ESP32

The ESP32 uses the Xtensa architecture and the Espressif Rust toolchain.

### Enter the project

```bash
cd firmware/esp32
```

The local `rust-toolchain.toml` automatically selects:

```toml
[toolchain]
channel = "esp"
```

### Check

Run a fast compilation check without producing a final firmware artifact:

```bash
cargo check
```

### Debug build

```bash
cargo build
```

### Release build

```bash
cargo build --release
```

The release profile is optimized for embedded use and is configured in the firmware's `Cargo.toml`.

## ESP32-C3

The ESP32-C3 uses the RISC-V architecture and the standard Rust toolchain.

### Enter the project

```bash
cd firmware/esp32c3
```

The local `rust-toolchain.toml` automatically configures the required Rust components and target:

```toml
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imc-unknown-none-elf"]
```

### Check

```bash
cargo check
```

### Debug build

```bash
cargo build
```

### Release build

```bash
cargo build --release
```

## Treating Warnings as Errors

The firmware should compile without warnings.

To enforce this locally:

### ESP32

```bash
RUSTFLAGS="-D warnings" cargo check --release
```

### ESP32-C3

```bash
RUSTFLAGS="-D warnings" cargo check --release
```

This is also enforced by the embedded CI builds.

Using `-D warnings` is useful for catching issues that may otherwise be ignored during development.

## Cargo Configuration

Each firmware contains a target-specific `.cargo/config.toml`.

The configuration is responsible for target-specific build settings such as:

* target architecture;
* linker configuration;
* runner;
* environment variables;
* unstable Cargo build settings.

For example, ESP32 uses:

```toml
[build]
target = "xtensa-esp32-none-elf"
```

while ESP32-C3 uses:

```toml
[build]
target = "riscv32imc-unknown-none-elf"
```

Because these configurations are target-specific, they should remain inside their respective firmware directories.

## Build Scripts

Both firmware targets contain a `build.rs`.

These scripts configure target-specific linker and build behavior required by the respective ESP32 targets.

The build scripts are intentionally kept separate because the targets have different toolchains and linker requirements.

When adding a new MCU target, its `build.rs` should be based on the requirements of that MCU rather than copied blindly from an existing target.

## Build from Docker

The same firmware commands can be executed inside the target-specific Docker containers.

### ESP32

```bash
docker compose run --rm esp32 cargo build --release
```

### ESP32-C3

```bash
docker compose run --rm esp32c3 cargo build --release
```

For a complete description of the Docker environments, see [DOCKER.md](DOCKER.md).

## Root Workspace

The root `Cargo.toml` contains the reusable shared crates:

```toml
[workspace]
resolver = "3"
members = [
    "crates/logger",
]
```

The firmware projects are intentionally not members of this workspace.

Consequently, firmware build commands should normally be executed from:

```text
firmware/esp32/
```

or:

```text
firmware/esp32c3/
```

rather than from the repository root.

This prevents target-specific firmware configuration from being mixed with the shared crates.

## Clean Builds

To remove generated build artifacts for the current firmware project:

```bash
cargo clean
```

Then rebuild:

```bash
cargo build --release
```

Cargo stores build artifacts in the project's `target/` directory.

These generated files are ignored by Git.

## Troubleshooting

### Wrong toolchain

Check the active Rust toolchain:

```bash
rustup show active-toolchain
```

For ESP32, the active toolchain should be the Espressif `esp` toolchain.

For ESP32-C3, the active toolchain should be `stable`.

The `rust-toolchain.toml` files normally configure this automatically.

### Missing target

For ESP32-C3, verify that the target is installed:

```bash
rustup target list --installed
```

The expected target is:

```text
riscv32imc-unknown-none-elf
```

The ESP32 target is provided by the Espressif toolchain rather than the standard Rust target list.

### Build cache problems

If a build behaves unexpectedly after changing toolchains or dependencies, try:

```bash
cargo clean
cargo build --release
```

Avoid deleting Cargo's global registry or Git cache unless necessary, as doing so forces all dependencies to be downloaded again.

### Dependency resolution problems

Each firmware project has its own `Cargo.lock`.

If dependencies have changed, update them from the corresponding firmware directory:

```bash
cargo update
```

Review the resulting changes before committing an updated lockfile.

## Adding a New Firmware Target

When adding a new MCU family, create a new independent firmware project:

```text
firmware/
├── esp32/
├── esp32c3/
└── new-target/
```

The new project should define its own:

* `Cargo.toml`
* `Cargo.lock`
* `rust-toolchain.toml`
* `.cargo/config.toml`
* `build.rs`
* firmware entry point

The build process should follow the conventions established by the existing targets while keeping MCU-specific configuration isolated.

If the target requires a new Docker environment, add a corresponding image under `docker/` and service to `compose.yml`.

CI should also receive a dedicated build job for the new target.

## Recommended Development Workflow

A typical firmware development cycle is:

```text
Edit source code
      │
      ▼
cargo check
      │
      ▼
cargo build
      │
      ▼
cargo build --release
      │
      ▼
cargo run --release
      │
      ▼
Flash and monitor
```

For changes that affect dependencies, toolchains or target configuration, perform a clean release build when appropriate.

Before committing firmware changes, verify:

```bash
cargo fmt --check
```

and:

```bash
RUSTFLAGS="-D warnings" cargo check --release
```

The exact CI checks are documented in the project's CI workflow.
