# ADR-0001: Report inspection capability and field availability

- Status: Accepted
- Date: 2026-11-23
- Decision owners: Zashikura project

## Context

Zashikura is cross-platform by architectural intent and Linux-first in development. It should also be usable inside Termux on an unrooted Android device. Termux runs in an Android application sandbox; it is not equivalent to a conventional desktop Linux installation. Kernel interfaces, process visibility, device metadata, storage topology, and Android framework services may be unavailable or restricted by Android permissions and SELinux.

The current system snapshot represents some unknown strings as `Option<String>`, but it cannot explain why a value is missing. A single all-or-nothing inspection result also does not adequately represent independent collectors or partial results.

## Decision

1. Keep platform-specific discovery in `zashikura-platform`; keep domain types independent of Android, Linux, Termux, and serialization.
2. Model field-level availability explicitly. At minimum distinguish:
   - `Available`: a value was collected.
   - `Restricted`: the platform denied or hid the information.
   - `Unsupported`: the platform or environment does not expose the capability.
   - `NotCollected`: the collector did not attempt it.
   - `Failed`: collection was attempted but failed for an operational reason.
3. Preserve partial results. A failure in one inspection category must not erase successful results from other categories.
4. Distinguish device-wide measurements from process/container-visible measurements. Never label a Termux-visible value as device-wide unless the source establishes that scope.
5. Do not require root, Android debugging, a companion application, or additional Android permissions for baseline inspection.
6. Optional Android integrations may be considered later, but must be separately permissioned, documented, and unavailable by default.
7. Do not collect hardware serial numbers, IMEI, MAC addresses, advertising identifiers, or other persistent personal/device identifiers by default.
8. Preserve existing CLI behavior while evolving the output schema deliberately. JSON output must expose availability and measurement scope rather than silently substituting zero or omitting why data is missing.

## Termux baseline

The initial unrooted Termux adapter may collect only information available to the Termux process through ordinary user-space interfaces. Candidate sources include accessible `/proc`, `/sys`, standard system calls, and existing platform libraries. Every source must be verified on supported Android versions and device configurations before being treated as reliable.

The adapter must not assume that:
- desktop Linux DMI, block-device, or hardware topology interfaces are readable;
- process listings include other Android applications;
- filesystem mounts represent all physical device partitions;
- memory figures visible to the process represent unrestricted device-wide memory;
- Android framework APIs or Termux:API are installed;
- root-only kernel and device interfaces are accessible.

## Consequences

- Core and application APIs need a reusable representation for field availability, measurement scope, and category-level partial outcomes.
- Independent collectors can be composed into a complete machine inspection without coupling unrelated categories.
- CLI text and JSON output can explain unavailable information consistently.
- Android/Termux support can be tested separately from desktop Linux without weakening Linux as the primary development target.
- Existing snapshots and output require a compatibility-conscious migration.

## Validation requirements

Before claiming Termux support, validate on an unrooted Android/Termux environment or a reproducible Android CI/device environment. Record Android version, architecture, Termux version, available permissions, and source used for each reported field. Linux desktop CI alone cannot establish Termux behavior.
