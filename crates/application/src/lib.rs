//! Interface-neutral application use cases and ports.
#![forbid(unsafe_code)]

pub use zashikura_core as core;

use std::error::Error;
use std::fmt::{Display, Formatter};
use zashikura_core::SystemSnapshot;

/// Read-only port implemented by an adapter for local machine inspection.
pub trait SystemInspector {
    fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError>;
}

/// Application use case for obtaining a read-only system snapshot.
pub struct InspectSystem<I> {
    inspector: I,
}

impl<I: SystemInspector> InspectSystem<I> {
    #[must_use]
    pub const fn new(inspector: I) -> Self {
        Self { inspector }
    }

    pub fn execute(&self) -> Result<SystemSnapshot, SystemInspectionError> {
        self.inspector.inspect_system()
    }
}

/// A failure reported by a system-inspection adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemInspectionError {
    message: String,
}

impl SystemInspectionError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for SystemInspectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for SystemInspectionError {}

#[cfg(test)]
mod tests {
    use super::{InspectSystem, SystemInspectionError, SystemInspector};
    use zashikura_core::{MemorySnapshot, SystemSnapshot};

    struct FakeInspector;
    impl SystemInspector for FakeInspector {
        fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError> {
            Ok(SystemSnapshot::new(
                Some("test-host".to_owned()),
                Some("test-os".to_owned()),
                None,
                None,
                4,
                MemorySnapshot::new(8_000, 2_000),
            ))
        }
    }

    struct FailingInspector;
    impl SystemInspector for FailingInspector {
        fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError> {
            Err(SystemInspectionError::new("inspection unavailable"))
        }
    }

    #[test]
    fn use_case_returns_the_adapter_snapshot() {
        let result = InspectSystem::new(FakeInspector).execute().unwrap();
        assert_eq!(result.hostname.as_deref(), Some("test-host"));
        assert_eq!(result.memory.total_bytes, 8_000);
    }

    #[test]
    fn use_case_preserves_adapter_errors() {
        let error = InspectSystem::new(FailingInspector).execute().unwrap_err();
        assert_eq!(error.to_string(), "inspection unavailable");
    }
}
