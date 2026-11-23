//! Operating-system adapters for local, read-only machine inspection.
#![forbid(unsafe_code)]

use sysinfo::System;
use zashikura_application::{SystemInspectionError, SystemInspector};
use zashikura_core::{MemorySnapshot, SystemSnapshot};

/// Cross-platform system-information adapter backed by sysinfo.
#[derive(Clone, Copy, Debug, Default)]
pub struct SysinfoSystemInspector;

impl SystemInspector for SysinfoSystemInspector {
    fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError> {
        let mut system = System::new_all();
        system.refresh_all();
        Ok(SystemSnapshot::new(
            System::host_name(),
            System::name(),
            System::os_version(),
            System::kernel_version(),
            system.cpus().len(),
            MemorySnapshot::new(system.total_memory(), system.used_memory()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::SysinfoSystemInspector;
    use zashikura_application::SystemInspector;

    #[test]
    fn inspection_returns_a_consistent_snapshot() {
        let snapshot = SysinfoSystemInspector.inspect_system().unwrap();
        assert!(snapshot.logical_cpu_count > 0);
        assert!(snapshot.memory.total_bytes > 0);
        assert!(snapshot.memory.used_bytes <= snapshot.memory.total_bytes);
    }
}
