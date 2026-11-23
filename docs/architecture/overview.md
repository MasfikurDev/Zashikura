# Architecture overview

## Goals

The architecture supports a CLI-first release without making the CLI the application itself. Future desktop interfaces, language bindings, automation wrappers, and MCP servers should reuse the same application capabilities.

The design follows ports and adapters. It should grow in response to real features rather than speculative framework requirements.

## Platform strategy

Zashikura is cross-platform by architectural intent, with Linux as the sole active development and validation target for now. The existing CLI is intended to remain usable on Linux, Windows, and macOS. Shared domain and application code must remain portable, and operating-system-specific behavior must stay behind platform adapters. Do not develop features or run routine validation on Windows or macOS at this stage. Non-Linux platform-specific implementation and verification are deferred until explicitly prioritized.

## Workspace

~~~
crates/
  core/          # platform-independent domain types and rules
  application/   # use cases and ports
  platform/      # operating-system adapters
  cli/           # first executable interface
docs/
  architecture/
  decisions/
~~~

Persistence is not a workspace member yet. Add a storage adapter only when a concrete persistence use case and its data lifecycle have been defined.

## Dependency direction

~~~
CLI / future UI / wrapper / MCP
              |
       Application API
          /       \
       Core       Ports
                   ^
             Platform adapters
~~~

The CLI and future interfaces are composition roots. They construct application use cases with the required adapters. Adapters depend on application-defined ports; the application layer never depends on platform implementations.

## Crate responsibilities

### zashikura-core

Owns platform-independent domain concepts, invariants, and rules. It must not depend on operating-system APIs, persistence, terminal presentation, serialization formats, or external protocols.

### zashikura-application

Owns interface-neutral use cases and the ports required to perform them. It coordinates domain logic and adapters without knowing whether a request came from the CLI, a desktop UI, a wrapper, or an MCP client.

### zashikura-platform

Owns operating-system-specific discovery and operations. Platform differences are isolated here and exposed through application ports. Implementations must state their platform support and failure behavior.

### zashikura-cli

Owns argument parsing, command routing, terminal output, exit codes, output serialization, and the CLI composition root. It calls application use cases rather than containing system logic.

## First vertical slice: read-only system inspection

The first application capability is read-only machine inspection. The core owns typed, platform-independent category models and explicit available, unavailable, and failed outcomes. The application crate defines the MachineInspector port and InspectMachine use case, which aggregates independent category results. The platform crate implements general discovery with sysinfo and narrowly scoped Linux hardware discovery through kernel sysfs interfaces. The CLI composes these parts. The original InspectSystem path remains available for the basic snapshot.

See [machine inspection architecture](machine-inspection.md) for category coverage, partial-result semantics, data sources, and extension rules. Output serialization and presentation remain at the CLI boundary.

## Cross-cutting rules

- Keep the application API independent of any transport or presentation format.
- Convert application results into CLI, JSON, MCP, or UI-specific representations at the relevant interface boundary.
- Keep serialization formats out of domain types.
- Version external protocols independently from internal Rust crate versions.
- Make read-only inspection the default. Model state-changing operations as explicit use cases with clear authorization and reporting.
- Do not add a plugin system until there is a concrete extension requirement.
