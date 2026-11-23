# Zashikura

Zashikura is a local-first tool for understanding and looking after your computer. It aims to make software, resources, activity, and their relationships easier to understand while keeping maintenance decisions under your control.

The project is CLI-first. The command-line interface is the first way to use Zashikura, not the boundary of its architecture. The core is being structured so future desktop interfaces, wrappers, automation integrations, and MCP support can reuse the same application capabilities.

## Project principles

- **Local by default.** Machine inventory, monitoring, relationships, and history stay on the machine unless a user explicitly chooses otherwise.
- **Transparent and controlled.** Inspection and modification are distinct. Actions that change system state must be clear and intentional.
- **Respect application-owned data.** Application-managed directories and data are left alone by default.
- **Useful, not intrusive.** Zashikura should help people understand and maintain their computers without aggressive optimization or unnecessary complexity.
- **Platform-aware, consistently designed.** Operating-system-specific behavior belongs behind platform adapters, not in the user interface or domain logic.

## Architecture

Zashikura is a Rust workspace with explicit boundaries:

| Crate | Responsibility |
| --- | --- |
| `zashikura-core` | Domain types, rules, and concepts. No operating-system or interface dependencies. |
| `zashikura-application` | Application use cases and ports consumed by interfaces and implemented by adapters. |
| `zashikura-platform` | Operating-system discovery and system-operation adapters. |
| `zashikura-storage` | Local persistence adapters and schema management. |
| `zashikura-cli` | CLI parsing, terminal presentation, and application composition. |

Future interfaces should depend on the application layer rather than implement their own system logic. A desktop UI, wrapper, or MCP server should be a separate entry point that composes the same application services and infrastructure adapters.

See [the architecture overview](docs/architecture/overview.md) and [extension points](docs/architecture/extension-points.md) before adding a new interface or integration.

## AI-assisted development

AI coding tools are supported through explicit repository guidance and documented architecture boundaries. AI assistance is not a runtime dependency, and no model or cloud service is required.

- [Agent instructions](AGENTS.md) define repository-wide engineering, privacy, and safety requirements.
- [AI-assisted development guide](docs/ai-assisted-development.md) explains how tools should navigate and extend the project.

AI-facing integrations such as MCP will be implemented as separate interfaces over the application API. They will not receive privileged or unrestricted system access by default.

## Development status

The workspace foundation is being established. The current CLI is intentionally small. System discovery, persistence, and maintenance capabilities will be added behind the boundaries above as their behavior and safety requirements are defined.

## Development

Install the stable Rust toolchain with the `rustfmt` and `clippy` components.

Check and test the workspace:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p zashikura-cli -- --help
```

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and the architecture documentation before making structural changes.

## License

Zashikura is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE).
