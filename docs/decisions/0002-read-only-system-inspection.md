# ADR 0002: Read-only system inspection as the first application capability

- **Status:** Accepted
- **Date:** 2026-11-23

## Context

The initial workspace established crate boundaries but contained no application behavior. A real vertical slice is needed to validate those boundaries before adding persistence, monitoring, maintenance actions, or additional interfaces.

## Decision

Implement a read-only system snapshot containing hostname, operating-system name and versions, logical processor count, and total and used physical memory.

- Domain snapshot types live in zashikura-core.
- The SystemInspector port and InspectSystem use case live in zashikura-application.
- The initial cross-platform adapter uses sysinfo in zashikura-platform.
- The CLI composes the use case and adapter and supports human-readable and JSON output.
- JSON serialization remains at the CLI boundary. Core and application types do not depend on serialization libraries.

The capability is read-only. It does not enumerate files, inspect application-owned directories, change system state, persist machine information, or transmit data.

## Consequences

- Future interfaces can reuse the same use case and adapter without depending on CLI code.
- Missing platform information is represented as an optional value.
- Adapter errors remain typed at the application boundary.
- The first capability validates the intended dependency direction with an executable, testable path.
