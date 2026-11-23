# Zashikura

Zashikura is a local-first tool for understanding and looking after your computer. It aims to make software, resources, activity, and their relationships easier to understand while keeping maintenance decisions under your control.

The project is CLI-first. The command-line interface is the first way to use Zashikura, not the boundary of its architecture. Future desktop interfaces, wrappers, automation integrations, and MCP support should reuse the same application capabilities.

## Project principles

- **Local by default.** Machine inventory, monitoring, relationships, and history stay on the machine unless a user explicitly chooses otherwise.
- **Transparent and controlled.** Inspection and modification are distinct. Actions that change system state must be clear and intentional.
- **Respect application-owned data.** Application-managed directories and data are left alone by default.
- **Useful, not intrusive.** Zashikura should help people understand and maintain their computers without aggressive optimization or unnecessary complexity.
- **Cross-platform by design, Linux-first in development.** Keep shared logic and interfaces portable, while targeting Linux for active implementation and validation. Do not develop or test features on Windows or macOS for now.

## Architecture

The Rust workspace has explicit boundaries:

| Crate | Responsibility |
| --- | --- |
| zashikura-core | Platform-independent domain types and rules. |
| zashikura-application | Interface-neutral use cases and ports. |
| zashikura-platform | Operating-system discovery and system-operation adapters. |
| zashikura-cli | CLI parsing, terminal presentation, and application composition. |

Future interfaces should call application use cases rather than implement their own system logic. Persistence is intentionally deferred until a concrete local-history or inventory requirement is designed.

See [the architecture overview](docs/architecture/overview.md) and [extension points](docs/architecture/extension-points.md) before adding a new interface or integration.

## AI-assisted development

AI coding tools are supported through explicit repository guidance and documented architecture boundaries. AI assistance is not a runtime dependency, and no model or cloud service is required.

- [Agent instructions](AGENTS.md) define repository-wide engineering, privacy, and safety requirements.
- [AI-assisted development guide](docs/ai-assisted-development.md) explains how tools should navigate and extend the project.

AI-facing integrations such as MCP will be implemented as separate interfaces over the application API. They will not receive privileged or unrestricted system access by default.

## Current capabilities

The initial read-only system inspection reports hostname, operating-system details, logical processor count, and memory usage. The CLI is intended to remain usable on Linux, Windows, and macOS for now. Linux is the only active development and CI validation environment; non-Linux feature work and platform-specific testing are deferred.

## Development

Install the stable Rust toolchain with the rustfmt and clippy components. Linux is the project's active development environment. Keep changes portable where practical, but do not use Windows or macOS as development or validation targets at this stage.

~~~
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p zashikura-cli -- --help
cargo run -p zashikura-cli -- inspect system
cargo run -p zashikura-cli -- inspect system --json
~~~

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and the architecture documentation before making structural changes.

## License

Zashikura is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE).
