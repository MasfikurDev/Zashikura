# Zashikura

Zashikura is a local-first tool for understanding and looking after your computer. It aims to make software, resources, activity, and their relationships easier to understand while keeping maintenance decisions under your control.

The project is CLI-first. The command-line interface is the first way to use Zashikura, not the boundary of its architecture. Future desktop interfaces, wrappers, automation integrations, and MCP support should reuse the same application capabilities.

## Project principles

- **Local by default.** Machine inventory, monitoring, relationships, and history stay on the machine unless a user explicitly chooses otherwise.
- **Transparent and controlled.** Inspection and modification are distinct. Actions that change system state must be clear and intentional.
- **Respect application-owned data.** Application-managed directories and data are left alone by default.
- **Useful, not intrusive.** Zashikura should help people understand and maintain their computers without aggressive optimization or unnecessary complexity.
- **Cross-platform by design, Linux-first in development.** Keep shared logic and interfaces portable, while targeting Linux for active implementation and validation. Do not develop or test features on Windows or macOS for now.

## Install

Download the matching package or portable archive from [GitHub Releases](https://github.com/MasfikurDev/Zashikura/releases). Qualifying changes merged into `main` produce automatic beta pre-releases; stable releases are published after an explicit maintainer release action. See [the release process](docs/releases.md).

| Linux architecture | Debian / Ubuntu | Fedora / RPM-based | GNU archive | Static standalone binary |
| --- | --- | --- | --- | --- |
| x86_64 | `zashikura_VERSION_amd64.deb` | `zashikura-VERSION-x86_64.rpm` | `zashikura-VERSION-linux-x86_64.tar.gz` | `zashikura-VERSION-linux-x86_64-musl` |
| ARM64 | `zashikura_VERSION_arm64.deb` | `zashikura-VERSION-aarch64.rpm` | `zashikura-VERSION-linux-aarch64.tar.gz` | `zashikura-VERSION-linux-aarch64-musl` |

Install a Debian package:

```sh
sudo apt install ./zashikura_VERSION_amd64.deb
```

Install an RPM package:

```sh
sudo dnf install ./zashikura-VERSION-1.x86_64.rpm
```

Install the portable archive:

```sh
tar -xzf zashikura-VERSION-linux-x86_64.tar.gz
sudo install -m 0755 zashikura-VERSION-linux-x86_64/zashikura /usr/local/bin/zashikura
```

Run a static standalone binary directly (no installation or Rust toolchain required):

```sh
chmod +x zashikura-VERSION-linux-x86_64-musl
./zashikura-VERSION-linux-x86_64-musl inspect machine
```

The matching `-musl.tar.gz` archive includes the binary, README, and license.

The static standalone binaries and their `-musl.tar.gz` archives do not require glibc or a Rust toolchain. They are intended to run directly on supported Linux architectures. The GNU packages and GNU archives require glibc 2.35 or newer. Replace `VERSION` and the architecture with the downloaded release's values. Verify downloaded files using the release's `SHA256SUMS` file before running or installing:

```sh
sha256sum -c SHA256SUMS
```

See [the release process](docs/releases.md) for supported artifacts and maintainer instructions.

## Architecture

The Rust workspace has explicit boundaries:

| Crate | Responsibility |
| --- | --- |
| zashikura-core | Platform-independent domain types and rules. |
| zashikura-application | Interface-neutral use cases and ports. |
| zashikura-platform | Operating-system discovery and system-operation adapters. |
| zashikura-cli | CLI parsing, terminal presentation, and application composition. |

Future interfaces should call application use cases rather than implement their own system logic. Persistence is intentionally deferred until a concrete local-history or inventory requirement is designed.

See [the architecture overview](docs/architecture/overview.md), [machine inspection architecture](docs/architecture/machine-inspection.md), and [extension points](docs/architecture/extension-points.md) before adding a new interface or integration.

## AI-assisted development

AI coding tools are supported through explicit repository guidance and documented architecture boundaries. AI assistance is not a runtime dependency, and no model or cloud service is required.

- [Agent instructions](AGENTS.md) define repository-wide engineering, privacy, and safety requirements.
- [AI-assisted development guide](docs/ai-assisted-development.md) explains how tools should navigate and extend the project.

AI-facing integrations such as MCP will be implemented as separate interfaces over the application API. They will not receive privileged or unrestricted system access by default.

## Current capabilities

The read-only machine inspection reports identity, processor, memory, storage, network, thermal, firmware/board, graphics, and power information when the platform exposes it. Categories are collected independently and report available, unavailable, or failed results without discarding other categories. The CLI provides `inspect system` for the original basic snapshot and `inspect machine` for the expanded report. Linux is the active development and CI validation environment; Linux-specific hardware discovery remains isolated in the platform crate.

## Development

Install the stable Rust toolchain with the rustfmt and clippy components. Linux is the project's active development environment. Keep changes portable where practical, but do not use Windows or macOS as development or validation targets at this stage.

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p zashikura-cli -- --help
cargo run -p zashikura-cli -- inspect system
cargo run -p zashikura-cli -- inspect system --json
cargo run -p zashikura-cli -- inspect machine
```

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and the architecture documentation before making structural changes.

## License

Zashikura is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE).
