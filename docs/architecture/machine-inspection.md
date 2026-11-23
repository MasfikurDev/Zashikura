# Machine inspection architecture

## Inspection model

Machine inspection is a read-only aggregation of independent categories. The application layer defines the `MachineInspector` port and `InspectMachine` use case. The platform adapter owns discovery mechanisms. The CLI invokes the use case and presents the resulting report.

The current categories are:

- **Identity:** hostname, operating system, OS and kernel versions, architecture, uptime.
- **Processor:** vendor and brand, logical and physical core counts, per-logical-processor name and reported frequency.
- **Memory:** physical total, used and available memory; swap total and used.
- **Storage:** discovered mounted devices, filesystem, mount point, capacity, available space, removable and read-only flags.
- **Network:** interface name and cumulative received/transmitted bytes and packets.
- **Thermal:** kernel thermal-zone labels and available temperature/threshold readings.
- **Firmware and board:** DMI system, product, board and BIOS fields when exposed by the kernel.
- **Graphics:** DRM card nodes, PCI vendor/device identifiers and bound driver when exposed by sysfs.
- **Power:** power-supply type, model, manufacturer, status and available capacity/electrical/energy readings.

## Partial results

Each category is represented by `Inspection<T>`:

- `Available(T)`: collection completed and returned data. An empty list means the source was readable and reported no entries.
- `Unavailable`: the source or measurement is not exposed or could not be read.
- `Failed(InspectionFailure)`: the collector encountered a category-specific failure with a diagnostic message.

The aggregator always returns every category result. A failure or unavailable result in one category does not discard successful results elsewhere. Missing measurements remain optional; they are never substituted with zero.

## Collection boundaries

The cross-platform adapter uses `sysinfo` for identity, CPU, memory, storage and network information. Linux-specific hardware details are read from narrowly scoped kernel interfaces under `/sys/class` (DMI, DRM, thermal and power-supply attributes). The sysinfo adapter refreshes only the CPU and memory data needed by this feature; it does not enumerate processes. These reads do not enumerate user files, inspect application-owned directories, or modify machine state.

Unsupported platform-specific categories report `Unavailable`. Do not infer hardware properties from model names or fill missing values with guesses.

## Extension rules

Add a category only when it has a concrete user-facing purpose, a typed core model, an application port method, a platform collector, and tests for both available and missing data. Keep collection logic out of the CLI. Avoid collecting processes, installed software, user files, or application data as part of machine inspection; those belong to separately scoped capabilities.
