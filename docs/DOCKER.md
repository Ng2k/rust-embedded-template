# Docker Development Environments

This project provides isolated Docker environments for building and developing the supported embedded firmware targets.

The Docker setup separates the common Rust environment from target-specific toolchains and tools.

## Architecture

The Docker images are organized in layers:

```text
rust-embedded-base
        │
        ├── rust-embedded-esp32
        │       ├── Espressif Xtensa toolchain
        │       └── espflash
        │
        └── rust-embedded-esp32c3
                ├── RISC-V target
                └── espflash
```

### Base image

`docker/base/Dockerfile` provides the common development environment:

* Debian Bookworm
* Rust installed through `rustup`
* `rust-src`
* Cargo
* Git
* Clang
* GCC / build tools
* OpenSSL development libraries
* `pkg-config`

The base image does not contain any MCU-specific toolchain.

### ESP32 image

`docker/esp32/Dockerfile` extends the base image with:

* Espressif's Rust toolchain through `espup`
* Xtensa ESP32 GCC toolchain
* `espflash`
* `libudev-dev`

The ESP32 uses the Xtensa architecture and therefore requires the Espressif Rust toolchain.

### ESP32-C3 image

`docker/esp32c3/Dockerfile` extends the base image with:

* `riscv32imc-unknown-none-elf`
* `espflash`

The ESP32-C3 uses the RISC-V architecture and can therefore use the standard Rust toolchain.

## Building the Images

Build the base image first:

```bash
docker build \
    -f docker/base/Dockerfile \
    -t rust-embedded-base:latest \
    .
```

Then build the target-specific images.

### ESP32

```bash
docker build \
    -f docker/esp32/Dockerfile \
    -t rust-embedded-esp32:latest \
    .
```

### ESP32-C3

```bash
docker build \
    -f docker/esp32c3/Dockerfile \
    -t rust-embedded-esp32c3:latest \
    .
```

The target-specific Dockerfiles use `rust-embedded-base:latest` as their parent image, so the base image must exist locally before they are built.

## Docker Compose

`compose.yml` provides development containers with the repository mounted at `/workspace`.

Start an ESP32 development container with:

```bash
docker compose run --rm esp32
```

Start an ESP32-C3 development container with:

```bash
docker compose run --rm esp32c3
```

The containers start in their corresponding firmware directory:

```text
/workspace/firmware/esp32
/workspace/firmware/esp32c3
```

The repository is mounted from the host:

```text
host repository
      │
      ▼
/workspace
```

Changes made on the host are therefore immediately available inside the container.

## Cargo Cache

The Compose configuration uses named Docker volumes for Cargo's registry and Git dependencies:

```yaml
volumes:
  - cargo-registry:/home/rust/.cargo/registry
  - cargo-git:/home/rust/.cargo/git
```

This prevents Cargo from downloading all dependencies again every time a container is recreated.

The cache is shared between the ESP32 and ESP32-C3 development containers.

## Building Firmware Inside Docker

Once inside the appropriate container, use the normal Cargo commands.

### ESP32

```bash
cargo check
cargo build
cargo build --release
```

### ESP32-C3

```bash
cargo check
cargo build
cargo build --release
```

The firmware-specific Cargo configuration automatically selects the correct target and linker configuration.

## Flashing from Docker

The Compose configuration passes the board's serial device into the corresponding container.

For example, the ESP32 service currently exposes:

```text
/dev/ttyUSB0
```

while the ESP32-C3 service exposes:

```text
/dev/ttyACM0
```

Inside the container, `cargo run` uses the `espflash` runner configured by the firmware's `.cargo/config.toml`.

### ESP32

```bash
cargo run --release
```

### ESP32-C3

```bash
cargo run --release
```

This builds the firmware, flashes the board and starts the serial monitor.

The device path in `compose.yml` may need to be changed depending on the USB-to-serial interface and board connected to the host.

## Toolchain Isolation

Each firmware target contains its own `rust-toolchain.toml`.

The ESP32 uses:

```toml
[toolchain]
channel = "esp"
```

The ESP32-C3 uses:

```toml
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imc-unknown-none-elf"]
```

This allows each target to manage its own Rust toolchain independently.

The Docker images follow the same principle:

```text
ESP32
    │
    └── Espressif Rust + Xtensa GCC

ESP32-C3
    │
    └── stable Rust + RISC-V target
```

This separation is intentional and should be preserved when adding new MCU targets.

## CI

GitHub Actions builds the firmware using the same Docker images used for local development.

The CI process is approximately:

```text
Checkout repository
        │
        ▼
Build base Docker image
        │
        ├───────────────┐
        ▼               ▼
ESP32 image       ESP32-C3 image
        │               │
        ▼               ▼
Cargo build       Cargo build
```

This keeps the compiler and embedded tooling used by CI aligned with the development environments.

CI does not flash physical hardware. It only verifies that the firmware can be built successfully.

## Troubleshooting

### Base image not found

If a target image fails with an error similar to:

```text
rust-embedded-base:latest: not found
```

build the base image first:

```bash
docker build \
    -f docker/base/Dockerfile \
    -t rust-embedded-base:latest \
    .
```

### Serial device not found

If flashing fails because the serial device does not exist, check which device was assigned to the board:

```bash
ls /dev/ttyUSB*
```

and:

```bash
ls /dev/ttyACM*
```

Update the corresponding device mapping in `compose.yml` if necessary.

### Permission denied on serial device

The Docker container must be able to access the host serial device.

On Linux, check the permissions of the device:

```bash
ls -l /dev/ttyUSB0
```

or:

```bash
ls -l /dev/ttyACM0
```

If necessary, ensure the host user has permission to access the serial device, commonly through the `dialout` group.

### Cargo dependencies are downloaded repeatedly

Check that the named Cargo volumes are present:

```bash
docker volume ls
```

The expected volumes are:

```text
cargo-registry
cargo-git
```

Removing these volumes forces Cargo to download dependencies again.

## Adding a New MCU

A new MCU should normally receive its own Docker image when its toolchain or build environment differs from the existing targets.

For example:

```text
docker/
├── base/
│   └── Dockerfile
├── esp32/
│   └── Dockerfile
├── esp32c3/
│   └── Dockerfile
└── stm32/
    └── Dockerfile
```

The new image should extend `rust-embedded-base` and contain only the additional tools required by that target.

The corresponding service can then be added to `compose.yml`.

This keeps the common environment centralized while allowing each MCU family to remain independent.
