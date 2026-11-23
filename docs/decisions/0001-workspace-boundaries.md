# ADR 0001: Rust workspace and interface boundaries

- **Status:** Accepted
- **Date:** 2026-11-23

## Context

Zashikura is CLI-first, but future desktop interfaces, wrappers, automation integrations, and MCP support should be possible without rewriting system logic or making the CLI a dependency of other interfaces.

## Decision

Use a Rust Cargo workspace with separate crates for:

- domain logic (`zashikura-core`)
- application use cases and ports (`zashikura-application`)
- operating-system adapters (`zashikura-platform`)
- local persistence (`zashikura-storage`)
- the initial CLI entry point (`zashikura-cli`)

The application layer exposes interface-neutral APIs. Each interface is an entry point that composes application services and the adapters it requires. Platform and storage implementations satisfy ports defined by the application layer.

## Consequences

- The CLI remains the first interface, not the owner of application behavior.
- Future interfaces can reuse application capabilities.
- Operating-system and database dependencies remain outside the domain layer.
- Additional crates and protocol dependencies are introduced only when a concrete feature requires them.
- External APIs and protocols are versioned at their boundaries rather than coupled to internal crate structure.
