# Rust Embedded Template

A minimal and reusable template for developing embedded firmware in Rust.

The project is built around the [esp-rs](https://github.com/esp-rs) ecosystem and currently provides working firmware templates for:

* ESP32 (Xtensa)
* ESP32-C3 (RISC-V)

The repository intentionally provides **infrastructure rather than a framework**. It includes project structure, target-specific toolchains, build configuration, logging, Docker environments and CI without introducing unnecessary abstractions.

## Goals

The template provides:

* A clean starting point for Rust embedded projects
* Support for multiple MCU architectures
* Isolated toolchains and build configurations per firmware target
* Reproducible development environments
* `no_std` firmware
* `defmt` logging
* Cargo-based dependency management
* Flashing and monitoring with `espflash`
* Docker-based development environments
* GitHub Actions CI
* `prek` pre-commit checks
* A structure that can be extended to additional MCU families

The template is intentionally designed to grow without coupling hardware-specific code to reusable application logic.

## Supported Targets

| Target   | Architecture | Toolchain | HAL       |
| -------- | ------------ | --------- | --------- |
| ESP32    | Xtensa       | `esp`     | `esp-hal` |
| ESP32-C3 | RISC-V       | `stable`  | `esp-hal` |

The two targets intentionally use different CPU architectures and Rust toolchains. This provides a practical example of how the template isolates target-specific build environments.

## Project Structure

```text
rust-embedded-template/
├── .github/
│   └── workflows/
│       └── ci.yml
│
├── crates/
│   └── logger/
│       ├── src/
│       │   └── lib.rs
│       ├── tests/
│       │   └── logging.rs
│       └── Cargo.toml
│
├── docker/
│   ├── base/
│   │   └── Dockerfile
│   ├── esp32/
│   │   └── Dockerfile
│   └── esp32c3/
│       └── Dockerfile
│
├── docs/
│   ├── BUILD.md
│   ├── CONTRIBUTING.md
│   └── DOCKER.md
│
├── firmware/
│   ├── esp32/
│   │   ├── .cargo/
│   │   │   └── config.toml
│   │   ├── src/
│   │   │   └── bin/
│   │   │       └── main.rs
│   │   ├── build.rs
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── rust-toolchain.toml
│   │
│   └── esp32c3/
│       ├── .cargo/
│       │   └── config.toml
│       ├── src/
│       │   └── bin/
│       │       └── main.rs
│       ├── build.rs
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── rust-toolchain.toml
│
├── .gitignore
├── .pre-commit-config.yaml
├── Cargo.lock
├── Cargo.toml
├── compose.yml
├── LICENSE
└── README.md
```

### Workspace Structure

The repository uses **independent Cargo projects** for the different firmware targets.

The root workspace contains reusable shared crates:

```toml
[workspace]
resolver = "3"
members = [
    "crates/logger",
]
```

Each firmware has its own workspace:

```toml
[workspace]
```

This is intentional. ESP32 and ESP32-C3 use different architectures and toolchains, so keeping their Cargo projects independent prevents target-specific configuration from leaking between them.

Build commands should therefore normally be executed from the corresponding firmware directory.

## Toolchains

The original ESP32 uses the Xtensa architecture and requires the Espressif Rust toolchain:

```toml
[toolchain]
channel = "esp"
```

ESP32-C3 uses RISC-V and the standard Rust toolchain:

```toml
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imc-unknown-none-elf"]
```

Each firmware contains its own `rust-toolchain.toml`, allowing the targets to be developed independently.

## Development Environment

The repository supports two development approaches:

* **Docker-based development**, recommended for a reproducible environment
* **Native development**, for developers who prefer installing the required embedded toolchains locally

The Docker environment provides the Rust toolchain, target-specific tools and build dependencies without requiring the embedded toolchains to be installed directly on the host system.

For detailed setup instructions, see [DOCKER.md](docs/DOCKER.md).

### Native Development

Native development requires:

* Rust and `rustup`
* `espup` for ESP32 Xtensa development
* `espflash`
* Git
* `prek` for local pre-commit checks

The exact requirements may change as the esp-rs ecosystem evolves. Refer to the official [esp-rs documentation](https://docs.esp-rs.org/) when setting up a new development machine.

#### Rust

Install Rust using [rustup](https://rustup.rs).

#### ESP32 Xtensa

Install `espup`:

```bash
cargo install espup
```

Then install the ESP32 toolchain:

```bash
espup install --targets esp32
```

The ESP32 firmware's `rust-toolchain.toml` automatically selects the `esp` toolchain.

#### ESP32-C3

ESP32-C3 uses the standard Rust toolchain. Its `rust-toolchain.toml` automatically installs the required `rust-src` component and `riscv32imc-unknown-none-elf` target.

## Building

Build commands should normally be executed from the corresponding firmware directory.

### ESP32

```bash
cd firmware/esp32
cargo check
cargo build
cargo build --release
```

### ESP32-C3

```bash
cd firmware/esp32c3
cargo check
cargo build
cargo build --release
```

The firmware-specific `.cargo/config.toml` files configure the appropriate target, linker settings and build options automatically.

For detailed build and flashing instructions, see [BUILD.md](docs/BUILD.md).

## Flashing and Monitoring

The firmware crates configure `espflash` as their Cargo runner.

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

For Docker-specific flashing instructions and serial device configuration, see [BUILD.md](docs/BUILD.md) and [DOCKER.md](docs/DOCKER.md).

## Logging

Embedded firmware uses [`defmt`](https://github.com/knurling-rs/defmt) for structured logging.

The template also provides a small `logger` crate that exposes a common logging interface while keeping the underlying logging implementation target-dependent.

Currently:

* Embedded builds use `defmt`
* Host builds use `tracing`

Example:

```rust
logger::info!("Hello world!");
```

The embedded logging path is:

```text
Application
    │
    ▼
  logger
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

The log level is configured through:

```toml
[env]
DEFMT_LOG = "info"
```

The `espflash` runner is configured to decode the `defmt` output automatically.

## Docker

The repository provides separate Docker environments for the common development layers:

```text
rust-embedded-base
        ├── rust-embedded-esp32
        └── rust-embedded-esp32c3
```

The base image contains the common Rust development environment.

The MCU-specific images add the required target toolchains and embedded tooling.

Docker Compose provides convenient development containers:

```bash
docker compose run --rm esp32
```

or:

```bash
docker compose run --rm esp32c3
```

The Docker environments are intended to make development reproducible across different host machines.

For detailed Docker usage, see [DOCKER.md](docs/DOCKER.md).

## Development Checks

The repository uses [`prek`](https://github.com/j178/prek) to run formatting, linting and tests locally before creating a commit.

Install `prek` with Cargo:

```bash
cargo install --locked prek
```

Install the Git pre-commit hook from the repository root:

```bash
prek install
```

The hook automatically runs:

* `cargo fmt --all -- --check`
* `cargo clippy --workspace --all-targets --no-default-features --features host -- -D warnings`
* `cargo test --workspace --no-default-features --features host`

To run all checks manually, including files that are not staged:

```bash
prek run --all-files
```

The local hooks provide fast feedback during development. GitHub Actions remains the authoritative validation for the repository and additionally builds the ESP32 and ESP32-C3 firmware targets.

For the complete contribution workflow, see [CONTRIBUTING.md](docs/CONTRIBUTING.md).

## CI

GitHub Actions validates the project automatically.

The CI performs:

* Rust formatting checks
* Clippy checks with warnings treated as errors
* Host-side compilation
* Host-side tests
* ESP32-C3 firmware build
* ESP32 firmware build

The embedded firmware builds are performed inside the same Docker environments used for local development.

Hardware flashing is intentionally **not** performed in CI.

## Design Philosophy

### Keep the Template Minimal

The repository should provide infrastructure, not become a framework.

Only abstractions that solve a recurring problem across projects should be introduced.

### Keep Hardware-Specific Code Isolated

As a project grows, application and reusable logic can remain independent of the underlying MCU whenever reasonably possible.

One possible architecture for larger projects is:

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

This diagram represents a possible direction for projects that grow beyond the initial template. It is **not a mandatory architecture**.

Hardware-specific implementation should remain isolated where practical.

### Prefer Standard Embedded Abstractions

Reusable drivers and interfaces should prefer ecosystem standards such as:

* `embedded-hal`
* `embedded-hal-async`

rather than depending directly on a specific MCU whenever practical.

These abstractions should only be introduced when they solve a real reuse problem.

### Keep `no_std`

Firmware and reusable embedded components should use `no_std` by default.

Features such as:

* dynamic allocation
* asynchronous runtimes
* networking
* RTOS integration

should only be introduced when required by the project.

## Adding New Targets

Additional MCU families can be added without modifying the existing firmware configurations.

For example:

```text
firmware/
├── esp32/
├── esp32c3/
├── stm32/
└── ...
```

A new MCU target should provide its own:

* `Cargo.toml`
* `Cargo.lock`
* `rust-toolchain.toml`
* `.cargo/config.toml`
* `build.rs`
* firmware entry point

Target-specific configuration should remain isolated.

Shared functionality should only be moved into `crates/` when there is a concrete reason to reuse it.

## Documentation

Detailed development instructions are maintained separately:

* [Docker Development Guide](docs/DOCKER.md) — Docker images, Compose and development containers
* [Firmware Build Guide](docs/BUILD.md) — building, flashing and monitoring firmware
* [Contributing Guide](docs/CONTRIBUTING.md) — development workflow, checks and contribution conventions

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

