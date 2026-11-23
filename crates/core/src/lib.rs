//! Platform-independent domain types for Zashikura.
#![forbid(unsafe_code)]

/// A read-only snapshot of basic machine information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemSnapshot {
    pub hostname: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub kernel_version: Option<String>,
    pub logical_cpu_count: usize,
    pub memory: MemorySnapshot,
}

/// Physical memory measurements, expressed in bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemorySnapshot {
    pub total_bytes: u64,
    pub used_bytes: u64,
}

impl SystemSnapshot {
    #[must_use]
    pub const fn new(
        hostname: Option<String>,
        os_name: Option<String>,
        os_version: Option<String>,
        kernel_version: Option<String>,
        logical_cpu_count: usize,
        memory: MemorySnapshot,
    ) -> Self {
        Self { hostname, os_name, os_version, kernel_version, logical_cpu_count, memory }
    }
}

impl MemorySnapshot {
    #[must_use]
    pub const fn new(total_bytes: u64, used_bytes: u64) -> Self {
        Self { total_bytes, used_bytes }
    }
}

#[cfg(test)]
mod tests {
    use super::{MemorySnapshot, SystemSnapshot};

    #[test]
    fn snapshots_preserve_unknown_platform_fields() {
        let snapshot = SystemSnapshot::new(
            None, Some("Example OS".to_owned()), None, None, 8,
            MemorySnapshot::new(16_000, 4_000),
        );
        assert_eq!(snapshot.hostname, None);
        assert_eq!(snapshot.os_name.as_deref(), Some("Example OS"));
        assert_eq!(snapshot.logical_cpu_count, 8);
        assert_eq!(snapshot.memory.used_bytes, 4_000);
    }
}
