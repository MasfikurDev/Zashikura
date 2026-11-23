# ADR 0001: Rust workspace and interface boundaries

- **Status:** Accepted
- **Date:** 2026-11-23

## Context

Zashikura is CLI-first, but future desktop interfaces, wrappers, automation integrations, and MCP support should be possible without rewriting system logic or making the CLI a dependency of other interfaces.

## Decision

Use a Rust Cargo workspace with separate crates for:

- platform-independent domain types and rules (zashikura-core)
- application use cases and ports (zashikura-application)
- operating-system adapters (zashikura-platform)
- the initial CLI entry point (zashikura-cli)

The application layer exposes interface-neutral APIs. Each interface is a composition root that constructs application use cases with the adapters it needs. Adapters implement ports defined by the application layer.

Do not create a storage crate until a concrete persistence use case, data lifecycle, and migration strategy have been designed.

## Consequences

- The CLI remains the first interface, not the owner of application behavior.
- Future interfaces can reuse application capabilities.
- Operating-system and database dependencies remain outside the domain layer.
- Additional crates and protocol dependencies are introduced only when a concrete feature requires them.
- External APIs and protocols are versioned at their boundaries rather than coupled to internal Rust crate structure.
