# Contributing

Thank you for contributing to the Rust Embedded Template.

This document describes the development workflow and conventions used by the project.

## Development Environment

The repository provides Docker-based development environments for the supported firmware targets.

See:

* [Docker Development Guide](DOCKER.md)
* [Firmware Build Guide](BUILD.md)

The Docker environments are the recommended way to obtain a reproducible embedded development environment.

## Repository Structure

The repository separates reusable code from target-specific firmware:

```text
crates/
    Shared reusable crates

firmware/
    Target-specific firmware

docker/
    Development environments

.github/
    GitHub Actions workflows
```

Firmware targets are independent Cargo projects because different MCU architectures may require different Rust toolchains and target configurations.

## Local Development Checks

The project uses [`prek`](https://github.com/j178/prek) for local pre-commit checks.

Install it with Cargo:

```bash
cargo install --locked prek
```

Install the Git hook from the repository root:

```bash
prek install
```

The pre-commit hook runs:

* `cargo fmt --all -- --check`
* `cargo clippy --workspace --all-targets --no-default-features --features host -- -D warnings`
* `cargo test --workspace --no-default-features --features host`

You can also run the checks manually:

```bash
prek run --all-files
```

Running the checks manually is useful when validating changes before staging or committing them.

## Rust Formatting

The project uses `rustfmt` for code formatting.

To check formatting:

```bash
cargo fmt --all -- --check
```

To automatically format the project:

```bash
cargo fmt --all
```

Formatting must pass before changes are merged.

## Clippy

Clippy is used to detect common mistakes and improve code quality.

Run the same check used by the pre-commit hook:

```bash
cargo clippy --workspace --all-targets --no-default-features --features host -- -D warnings
```

Warnings are treated as errors.

## Tests

Run the host-side test suite with:

```bash
cargo test --workspace --no-default-features --features host
```

Embedded firmware is not flashed or executed as part of the host test suite.

## Firmware Validation

Each firmware target is an independent Cargo project.

For example:

```bash
cd firmware/esp32
cargo check
cargo build --release
```

or:

```bash
cd firmware/esp32c3
cargo check
cargo build --release
```

For flashing and monitoring instructions, see [BUILD.md](BUILD.md).

## Docker

Docker images are provided for the supported embedded targets.

The base image contains the common Rust development environment, while target-specific images provide the required MCU toolchains and embedded tooling.

See [DOCKER.md](DOCKER.md) for details.

## Continuous Integration

GitHub Actions is the authoritative repository validation.

CI performs:

* Rust formatting checks
* Clippy checks
* Host-side compilation
* Host-side tests
* ESP32 firmware build
* ESP32-C3 firmware build

The embedded firmware builds run inside the same Docker environments used for local development.

Hardware flashing is intentionally not performed in CI.

Local `prek` checks are intended to provide fast feedback before changes are committed, while CI provides the final validation in a clean environment.

## Commit Messages

The project follows the [Conventional Commits](https://www.conventionalcommits.org/) specification.

Commit messages use the following format:

```text
<type>: <description>
```

Common types used by this repository are:

| Type       | Purpose                               |
| ---------- | ------------------------------------- |
| `feat`     | Add a new feature                     |
| `fix`      | Fix a bug                             |
| `refactor` | Change code without changing behavior |
| `docs`     | Documentation changes                 |
| `test`     | Add or modify tests                   |
| `ci`       | CI or repository automation changes   |
| `build`    | Build system or dependency changes    |
| `chore`    | Maintenance changes                   |
| `perf`     | Performance improvements              |

Examples:

```text
feat: add ESP32-C3 firmware template
fix: handle logger initialization failure
refactor: isolate firmware workspace
docs: document development workflow
ci: add prek pre-commit hooks
```

Keep commit descriptions short and written in the imperative style where practical.

## Pull Requests

Before opening a pull request:

1. Make sure the working tree contains only the intended changes.
2. Run the local `prek` checks.
3. Build any affected firmware targets.
4. Review the final diff.
5. Use a Conventional Commit message for the changes.

For example:

```bash
git status
prek run --all-files
git diff
```

Pull requests should keep changes focused and avoid unrelated modifications.

## Adding a New MCU Target

New MCU families should be added as independent firmware projects when their toolchain or target configuration differs from existing targets.

A new target should normally provide:

* `Cargo.toml`
* `Cargo.lock`
* `rust-toolchain.toml`
* `.cargo/config.toml`
* `build.rs`
* Firmware entry point

For example:

```text
firmware/
├── esp32/
├── esp32c3/
└── stm32/
```

Target-specific configuration should remain isolated from the existing firmware projects.

Reusable functionality should only be moved into `crates/` when there is a concrete reason to share it.

## Design Principles

Contributions should preserve the project's core principles:

* Keep the template minimal.
* Prefer infrastructure over framework-level abstractions.
* Keep hardware-specific code isolated.
* Prefer standard embedded ecosystem abstractions where practical.
* Keep firmware and reusable embedded components `no_std` by default.
* Avoid introducing dependencies without a clear benefit.
* Keep development environments reproducible.
* Preserve independent firmware toolchains where required.

When in doubt, prefer the simplest solution that solves the actual problem.

## License

By contributing to this repository, you agree that your contributions will be licensed under the project's [MIT License](../LICENSE).
