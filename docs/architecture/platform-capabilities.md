# Platform capability and Termux inspection design

## Purpose

Define how Zashikura reports comprehensive machine information across environments with different visibility. The immediate target is unrooted Termux on Android, while retaining desktop Linux as the primary development and validation platform.

This document defines the contract and implementation sequence. It does not claim that the categories below are already implemented.

## Platform identity

The platform adapter must distinguish at least:
- Desktop/server Linux.
- Android userspace running inside Termux.
- Other operating systems (future adapters).

Termux must not be identified as ordinary desktop Linux solely because it uses a Linux kernel. Platform detection belongs in `zashikura-platform`, not in core or application.

## Availability contract

Every optional inventory value must carry an explicit availability state:

| State | Meaning |
| --- | --- |
| Available | A value was collected successfully. |
| Restricted | The environment or permission boundary prevented access. |
| Unsupported | The platform does not provide this capability through the adapter. |
| NotCollected | The collector did not attempt this field. |
| Failed | Collection was attempted but failed for an operational reason. |

A value is present only for `Available`. Missing data must never be encoded as zero, an empty string, or an invented fallback.

Where applicable, measurements must also identify their scope:
- Device-wide.
- Process-visible.
- Container/application-sandbox-visible.
- Unknown.

The model should preserve source/provenance where it materially affects trust or interpretation, without exposing sensitive paths or identifiers in default output.

## Inspection outcome

Inspection categories are independent. A category result must be able to report:
- Successful values.
- Unavailable fields and their states.
- A concise, safe reason when useful.
- Whether the category completed fully or partially.

A complete machine inspection aggregates category results. Failure in one category must not discard successful results from another. A total failure should be distinguishable from a partial result and from a successful result containing unsupported fields.

## Termux baseline capability matrix

This matrix is a target contract, not a promise that every Android device exposes the information. The adapter must verify each source at runtime and report availability honestly.

| Category | Unrooted Termux baseline | Important boundary |
| --- | --- | --- |
| Android/device identity | Partial | Android build properties may be visible; stable hardware identifiers are excluded. |
| OS and kernel | Partial to broad | Android release/build and kernel information depend on accessible interfaces. |
| CPU | Partial | Architecture and logical processor count are likely; topology, per-core frequency and detailed features may be restricted or vendor-specific. |
| Memory | Partial | Process-visible and device-wide figures must not be conflated. |
| Storage | Partial | Termux/shared-storage mounts may be visible; physical devices and protected partitions are generally not. |
| Battery/power | Optional/limited | May require Android framework integration or Termux:API; not a baseline dependency. |
| GPU/graphics | Limited | Detailed device and driver information is not guaranteed through ordinary userspace. |
| Board/firmware | Usually restricted | DMI/firmware-style desktop interfaces are generally not available to an unprivileged Android app. |
| Network interfaces | Partial | Visible interfaces are not equivalent to full radio/modem or system network diagnostics. |
| Processes | Sandbox-limited | Do not assume visibility into other applications' processes or private metadata. |
| Installed applications | Not baseline | Android package visibility restrictions apply; do not enumerate apps through private or privileged mechanisms. |

## Collection rules

- No root, `su`, ADB, or privileged helper is required or invoked.
- No companion app or Android permission is required for baseline inspection.
- Do not read other applications' private directories or application-owned data.
- Do not inspect personal files as a side effect of machine inventory.
- Do not collect serial numbers, IMEI, MAC addresses, advertising IDs, or other persistent device identifiers by default.
- Do not transmit inventory or telemetry.
- Treat `/proc`, `/sys`, command output, and system properties as untrusted and potentially absent or malformed.
- Avoid shelling out where a stable library or direct read-only interface is sufficient. If a command is required, use a fixed executable and argument list; never interpolate user input.
- Do not report a process-visible measurement as device-wide.
- Keep optional Android framework/Termux:API integration as a separately designed capability with explicit permissions and clear user control.

## Architecture

Keep the existing four-crate workspace:
- `zashikura-core`: typed inventory values, availability, measurement scope, and invariants.
- `zashikura-application`: independent inspection use cases/ports and partial-result composition.
- `zashikura-platform`: Linux and Termux discovery adapters.
- `zashikura-cli`: commands and text/JSON presentation.

Do not add an Android-specific core crate or make the application layer depend on Android APIs. Do not add persistence for this work.

## Implementation sequence

1. Introduce field availability and measurement-scope domain types with unit tests.
2. Define independent category result types and application ports/use cases.
3. Preserve the existing `inspect system` command while introducing a richer, compatibility-conscious result.
4. Implement and test the Linux collectors first.
5. Add Termux detection and a conservative unrooted adapter using only verified, ordinary-user interfaces.
6. Add a `inspect machine` aggregate and category-specific commands; make partial results explicit in text and JSON.
7. Validate Android behavior on an actual unrooted Termux device or reproducible Android environment before marking support verified.

## Termux acceptance criteria

- The binary can run in Termux without root.
- Baseline inspection does not request additional Android permissions or require Termux:API.
- A restricted or absent source is represented accurately, not treated as zero.
- Successful categories survive failures in other categories.
- Process-visible and device-wide measurements are clearly distinguished.
- No protected application data, personal files, or persistent device identifiers are collected.
- JSON output is stable, documented, and includes availability/scope.
- Desktop Linux behavior remains covered by Linux CI.
- Termux support is not called verified until exercised in a real unrooted Termux environment.
