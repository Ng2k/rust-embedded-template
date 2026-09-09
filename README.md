# Rust Embedded Template

A minimal and reusable template for developing embedded firmware in Rust.

The project is designed around the [esp-rs](https://github.com/esp-rs) ecosystem and currently provides working firmware templates for both the original ESP32 (Xtensa) and ESP32-C3 (RISC-V).

The repository is intentionally kept minimal: it provides the project structure, toolchains, build configuration, logging and development workflow without introducing unnecessary framework-level abstractions.

## Goals

This template aims to provide:

* A clean starting point for Rust embedded projects
* Support for multiple MCU architectures
* Isolated toolchains per firmware target
* Reproducible development environments
* `no_std` firmware
* `defmt` logging
* Cargo-based dependency management
* Easy flashing with `espflash`
* A structure that can be extended to other MCU families such as STM32

The template is designed to grow with the project while keeping the hardware-specific parts isolated from reusable application code.

## Supported Targets

| Target   | Architecture | Rust Toolchain | HAL       |
| -------- | ------------ | -------------- | --------- |
| ESP32    | Xtensa       | `esp`          | `esp-hal` |
| ESP32-C3 | RISC-V       | `stable`       | `esp-hal` |

The two targets intentionally cover different CPU architectures.

This is useful because the template is not tied to a single instruction set or compiler toolchain.

## Project Structure

```text
rust-embedded-template/
├── firmware/
│   ├── esp32/
│   │   ├── .cargo/
│   │   │   └── config.toml
│   │   ├── src/
│   │   │   └── bin/
│   │   │       └── main.rs
│   │   ├── build.rs
│   │   ├── Cargo.toml
│   │   └── rust-toolchain.toml
│   │
│   └── esp32c3/
│       ├── .cargo/
│       │   └── config.toml
│       ├── src/
│       │   ├── bin/
│       │   │   └── main.rs
│       │   └── lib.rs
│       ├── build.rs
│       ├── Cargo.toml
│       └── rust-toolchain.toml
│
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

The root `Cargo.toml` defines the workspace, while each firmware crate contains its own target-specific configuration.

## Why Separate Toolchains?

ESP32 and ESP32-C3 use different CPU architectures.

The original ESP32 uses Xtensa and requires the Espressif Rust toolchain:

```text
esp
```

ESP32-C3 uses RISC-V and can use the standard Rust toolchain:

```text
stable
```

For this reason, each firmware crate has its own `rust-toolchain.toml`.

### ESP32

```toml
[toolchain]
channel = "esp"
```

### ESP32-C3

```toml
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imc-unknown-none-elf"]
```

This allows each firmware to be developed independently while still belonging to the same Cargo workspace.

> Because the workspace contains firmware using different Rust toolchains, build commands should normally be executed from the individual firmware directories rather than relying on root-level `cargo check --workspace`.

## Prerequisites

Install the following tools:

* Rust / `rustup`
* `espup`
* `espflash`
* Git

### Rust

Install Rust using `rustup`:

```text
https://rustup.rs
```

### espup

Install `espup`:

```bash
cargo install espup
```

Then install the Espressif toolchain:

```bash
espup install
```

For ESP32 and ESP32-C3 targets, the required toolchains can be installed with:

```bash
espup install --targets esp32,esp32c3
```

The exact installation requirements may change with newer versions of the esp-rs ecosystem. Refer to the official esp-rs documentation when setting up a new development machine.

## Building

### ESP32

Enter the ESP32 firmware directory:

```bash
cd firmware/esp32
```

Check the project:

```bash
cargo check
```

Build:

```bash
cargo build
```

Build an optimized release:

```bash
cargo build --release
```

### ESP32-C3

Enter the ESP32-C3 firmware directory:

```bash
cd firmware/esp32c3
```

Check the project:

```bash
cargo check
```

Build:

```bash
cargo build
```

Build an optimized release:

```bash
cargo build --release
```

## Flashing and Running

The firmware crates configure Cargo runners using `espflash`.

With a supported board connected through USB:

### ESP32

```bash
cd firmware/esp32
cargo run
```

### ESP32-C3

```bash
cd firmware/esp32c3
cargo run
```

For an optimized release build:

```bash
cargo run --release
```

`cargo run` builds the firmware, flashes the device and starts the serial monitor.

## Logging

The template uses [`defmt`](https://github.com/knurling-rs/defmt) for embedded logging.

The current logging stack is:

```text
Application
    │
    ▼
  defmt
    │
    ▼
esp-println
    │
    ▼
defmt-espflash
    │
    ▼
 espflash
```

A simple log message looks like:

```rust
use defmt::info;

info!("Hello World!");
```

The log level is configured through:

```toml
[env]
DEFMT_LOG = "info"
```

The `espflash` runner is configured to decode `defmt` output:

```toml
runner = "espflash flash --monitor --chip esp32 --log-format defmt"
```

or, for ESP32-C3:

```toml
runner = "espflash flash --monitor --chip esp32c3 --log-format defmt"
```

The intention is to keep application code independent from the transport used to display logs.

A project-specific logging abstraction can be introduced later if the application grows enough to require it.

## Workspace

The repository uses a Cargo workspace:

```toml
[workspace]
resolver = "3"
members = [
    "firmware/esp32",
    "firmware/esp32c3",
]
```

Common Cargo profiles are defined at the workspace level.

Firmware-specific configuration remains inside each crate:

* target architecture
* linker configuration
* runner
* environment variables
* Rust toolchain

This separation makes it possible to add additional firmware targets without coupling their build configuration.

## Design Philosophy

The template follows a few simple principles.

### Keep the template minimal

The repository should provide infrastructure, not become a framework.

Only abstractions that solve a recurring problem across projects should be added.

### Separate hardware from application logic

As a project grows, the intended architecture is:

```text
┌───────────────────────────┐
│        Application        │
├───────────────────────────┤
│          Domain           │
├───────────────────────────┤
│   Interfaces / Traits     │
├───────────────────────────┤
│         Drivers           │
├───────────────────────────┤
│           HAL             │
├───────────────────────────┤
│          Hardware         │
└───────────────────────────┘
```

Application and domain code should remain as independent from the underlying MCU as reasonably possible.

Hardware-specific code belongs at the bottom of the stack.

### Prefer standard embedded abstractions

Reusable drivers and interfaces should prefer ecosystem standards such as:

* `embedded-hal`
* `embedded-hal-async`

rather than depending directly on a specific MCU whenever practical.

### Keep `no_std`

Firmware and reusable embedded components should be designed around `no_std` by default.

Features such as allocation, asynchronous runtimes or networking should only be introduced when they are actually required.

## Future Targets

The repository is intended to support additional MCU families in the future.

For example:

```text
firmware/
├── esp32/
├── esp32c3/
├── stm32/
└── ...
```

An STM32 implementation would use its own HAL and toolchain configuration without changing the ESP32 firmware configuration.

The goal is to share the higher-level project architecture while keeping hardware-specific implementations isolated.

## Development Environment

The project can be developed locally using the installed Rust and Espressif tooling.

A Docker / Dev Container environment can also be provided to make the toolchain reproducible across development machines.

The intended layering is:

```text
rust-embedded-base
        │
        └── rust-embedded-esp
```

Additional MCU-specific environments can be added later, for example:

```text
rust-embedded-base
        ├── rust-embedded-esp
        └── rust-embedded-stm32
```

## License

See [LICENSE](LICENSE).
