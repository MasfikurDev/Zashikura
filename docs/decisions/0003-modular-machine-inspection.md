# ADR 0003: Modular machine inspection with partial results

- **Status:** Accepted
- **Date:** 2026-11-23

## Context

The initial system snapshot covered only basic operating-system, processor-count and memory values. The next machine inventory capability needs broader detail without coupling categories or treating inaccessible information as zero.

## Decision

Model machine inspection as independently collected categories coordinated by one application use case. The core owns typed category models and an explicit `Inspection<T>` state with Available, Unavailable and Failed outcomes. The application layer owns the category collector port and aggregate use case. The platform crate owns discovery and platform-specific access. The CLI remains a presentation and composition boundary.

The initial categories are identity, processor, memory, storage, network, thermal, firmware/board, graphics and power. Use `sysinfo` for general system resources and narrowly scoped Linux sysfs reads for hardware fields not exposed by that library.

Keep the existing basic `inspect system` command available. Add `inspect machine` for the aggregate report.

## Consequences

- One inaccessible category does not invalidate successful categories.
- Missing values are explicit and are not fabricated as zero.
- Linux-specific discovery remains isolated in the platform crate.
- Inspection remains read-only and local; no persistence, telemetry, network transmission, user-file scanning or application-owned data inspection is introduced.
- New categories require typed models, a port method, an adapter, tests and documentation.
