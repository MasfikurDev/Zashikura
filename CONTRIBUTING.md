# Contributing

Contributions should preserve Zashikura's local-first design, clear user control, and respect for application-owned data.

## Before changing architecture

For a new crate, interface, platform adapter, persistence mechanism, or integration, explain the need and its dependency direction in an issue or pull request. Avoid adding abstractions for hypothetical features without a concrete use case.

## Dependency boundaries

- Domain logic belongs in `zashikura-core`.
- Application workflows belong in `zashikura-application`.
- Operating-system access belongs in `zashikura-platform`.
- Add a storage adapter only when a concrete persistence use case and data lifecycle are defined; keep it behind application ports.
- Command parsing and terminal presentation belong in `zashikura-cli`.
- New interfaces should call application capabilities. They must not duplicate domain rules or access operating-system APIs directly.

Keep operating-system-specific behavior out of the core and application crates. Keep terminal formatting out of application services.

## Safety and privacy

- Treat inspection and modification as different capabilities.
- Do not modify files, settings, services, or processes without a clearly defined user-authorized action.
- Do not inspect application-owned directories by default.
- Do not transmit machine inventory, file information, or activity data.
- Make data collection, retention, and deletion behavior explicit.

## Validation

Before submitting a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Keep changes focused and include tests for observable behavior.
