# Instructions for AI coding agents

These instructions apply to the entire repository. Follow them alongside the user's request and the relevant architecture documentation.

## Platform development policy

- Linux is the only active development and validation platform for now. Implement and test actual features on Linux.
- Keep the architecture, shared Rust code, and CLI portable. The existing CLI commands are intended to remain usable on Linux, Windows, and macOS.
- Do not perform feature development, platform-specific implementation, or routine validation on Windows or macOS. Do not add non-Linux CI jobs at this stage.
- Avoid Linux-only assumptions in shared code. Isolate Linux-specific behavior behind platform adapters so other operating systems can be supported later.
- When a feature genuinely requires non-Linux-specific behavior, document the gap and defer that implementation rather than silently narrowing the shared API.

## Before changing code

1. Read this file and the relevant sections of `README.md`.
2. Inspect the current implementation and tests. Do not assume documentation describes code that has not been implemented.
3. Read `docs/architecture/overview.md`, `docs/architecture/extension-points.md`, and relevant architecture decision records before changing boundaries.
4. Identify the owning crate and the direction of its dependencies before adding code or dependencies.
5. State uncertainty explicitly. Do not invent existing behavior, platform support, command output, or test results.

## Architectural boundaries

- `zashikura-core`: domain concepts, invariants, and rules. Keep it independent of operating-system APIs, persistence, CLI frameworks, UI frameworks, and external protocols.
- `zashikura-application`: interface-neutral use cases and ports. This is the shared application API for all entry points.
- `zashikura-platform`: operating-system discovery and operations. Isolate platform-specific behavior here.
- Future storage adapters: local persistence and migrations, introduced only when a concrete persistence use case exists. Do not leak database types into domain or interface APIs.
- `zashikura-cli`: command parsing, terminal presentation, exit codes, and CLI composition only.

Future UI applications, wrappers, automation integrations, and MCP servers must call application use cases. They must not duplicate system logic or make the application layer depend on their transport or presentation framework.

Do not add empty crates, speculative plugin systems, or dependencies merely to reserve future possibilities. Add an extension point when a concrete capability requires it.

## Privacy and safety requirements

- Keep machine inventory, monitoring, relationships, and history local by default.
- Never transmit personal files, file contents, machine inventory, or activity data unless the user explicitly authorizes a clearly described feature.
- Do not infer or judge the user's intent from their files or activity.
- Respect application-owned directories and data. Do not inspect or modify them by default.
- Separate read-only inspection from state-changing operations.
- Never make destructive or privileged changes implicitly. State-changing use cases must have explicit scope, authorization, and understandable results.
- Treat data from the operating system, files, processes, and external tools as untrusted input.
- Do not add telemetry, cloud dependencies, AI services, or network communication without an explicit product decision.

## Implementation standards

- Prefer small, cohesive changes over broad rewrites.
- Preserve existing behavior unless the request explicitly changes it.
- Use typed domain and application interfaces. Avoid passing loosely structured maps through the core.
- Keep errors actionable and preserve useful error context.
- Add tests for behavior, edge cases, failure paths, and safety constraints.
- Document public APIs and non-obvious design decisions.
- Update architecture decision records when changing an established boundary.
- Keep dependencies minimal and justify new direct dependencies.
- Do not claim a command or test passed unless it was actually run and its result was observed.

## Validation

Run the checks relevant to the change:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Run development and validation on Linux only. Keep shared code portable, but clearly distinguish intended CLI portability from verified platform support. Do not claim Windows or macOS validation unless it was explicitly performed in a separately authorized effort.

## Working with the user

- Follow the requested scope. Do not silently broaden it.
- Explain meaningful trade-offs before introducing architectural complexity.
- Ask only when a missing decision materially blocks safe implementation.
- Summarize changed files, behavior, validation performed, and remaining limitations in the final report.
