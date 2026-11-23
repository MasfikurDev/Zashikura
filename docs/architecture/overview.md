# Architecture overview

## Goals

The architecture must support a CLI-first release without making the CLI the application itself. Future interfaces such as a desktop UI, language bindings, automation wrappers, and an MCP server should reuse the same application capabilities.

The design follows a ports-and-adapters approach with a small number of explicit Rust crates. It should grow in response to real features rather than speculative framework requirements.

## Workspace

```text
crates/
  core/          # domain model and rules
  application/   # use cases and ports
  platform/      # operating-system adapters
  storage/       # local persistence adapters
  cli/           # first executable interface
docs/
  architecture/
  decisions/
```

## Dependency direction

```text
                 CLI
                  |
          Application API
             /       \
          Core      Ports
                     ^
              +------+------+
              |             |
          Platform       Storage
           adapter       adapter

 Future UI / wrapper / MCP entry points
                  |
          Application API
```

The diagram describes dependency direction, not a requirement that every crate depend on every other crate. Adapters implement ports defined by the application layer. The CLI and future interfaces compose the required adapters at their entry points.

## Crate responsibilities

### `zashikura-core`

Owns domain concepts, invariants, and rules. It must not depend on a terminal framework, a database, a web framework, an MCP SDK, or operating-system APIs.

### `zashikura-application`

Owns use cases and the ports required to perform them. It exposes interface-neutral Rust APIs. It coordinates domain logic and adapters without knowing whether a request came from the CLI, a desktop UI, a wrapper, or an MCP client.

### `zashikura-platform`

Owns operating-system-specific discovery and operations. Platform differences are isolated here and exposed through application ports. Implementations must state their platform support and failure behavior.

### `zashikura-storage`

Owns local persistence, migrations, and storage adapters. Database types must not leak into domain or interface APIs. Persistence remains local by default.

### `zashikura-cli`

Owns argument parsing, command routing, terminal output, exit codes, and the composition root for the CLI executable. It should call application use cases rather than contain system logic.

## Cross-cutting rules

- Keep the application API independent of any transport or presentation format.
- Convert application results into CLI, JSON, MCP, or UI-specific representations at the interface boundary.
- Keep serialization formats out of domain types unless a concrete interoperability requirement justifies them.
- Version external protocols independently from internal Rust crate versions.
- Make read-only inspection the default. Model state-changing operations as explicit use cases with clear authorization and reporting.
- Do not add a plugin system until there is a concrete extension requirement.
