#![forbid(unsafe_code)]
use std::{
    error::Error,
    fmt::{Display, Formatter},
};
pub use zashikura_core as core;
use zashikura_core::{
    FirmwareInfo, GraphicsDevice, Inspection, MachineIdentity, MachineSnapshot, MemoryInfo,
    NetworkInterface, PowerSupply, ProcessorInfo, StorageDevice, SystemSnapshot, ThermalSensor,
};
pub trait SystemInspector {
    fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError>;
}
pub trait MachineInspector {
    fn inspect_identity(&self) -> Inspection<MachineIdentity>;
    fn inspect_processor(&self) -> Inspection<ProcessorInfo>;
    fn inspect_memory(&self) -> Inspection<MemoryInfo>;
    fn inspect_storage(&self) -> Inspection<Vec<StorageDevice>>;
    fn inspect_network(&self) -> Inspection<Vec<NetworkInterface>>;
    fn inspect_thermal(&self) -> Inspection<Vec<ThermalSensor>>;
    fn inspect_firmware(&self) -> Inspection<FirmwareInfo>;
    fn inspect_graphics(&self) -> Inspection<Vec<GraphicsDevice>>;
    fn inspect_power(&self) -> Inspection<Vec<PowerSupply>>;
}
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
pub struct InspectMachine<I> {
    inspector: I,
}
impl<I: MachineInspector> InspectMachine<I> {
    #[must_use]
    pub const fn new(inspector: I) -> Self {
        Self { inspector }
    }
    pub fn execute(&self) -> MachineSnapshot {
        MachineSnapshot {
            identity: self.inspector.inspect_identity(),
            processor: self.inspector.inspect_processor(),
            memory: self.inspector.inspect_memory(),
            storage: self.inspector.inspect_storage(),
            network: self.inspector.inspect_network(),
            thermal: self.inspector.inspect_thermal(),
            firmware: self.inspector.inspect_firmware(),
            graphics: self.inspector.inspect_graphics(),
            power: self.inspector.inspect_power(),
        }
    }
}
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl Error for SystemInspectionError {}

#[cfg(test)]
mod tests {
    use super::{InspectMachine, MachineInspector};
    use zashikura_core::{
        FirmwareInfo, GraphicsDevice, Inspection, MachineIdentity, MemoryInfo, NetworkInterface,
        PowerSupply, ProcessorInfo, StorageDevice, ThermalSensor,
    };
    struct PartialInspector;
    impl MachineInspector for PartialInspector {
        fn inspect_identity(&self) -> Inspection<MachineIdentity> {
            Inspection::available(MachineIdentity {
                hostname: Some("host".into()),
                operating_system: Some("Linux".into()),
                os_version: None,
                kernel_version: None,
                architecture: Some("x86_64".into()),
                uptime_seconds: Some(1),
            })
        }
        fn inspect_processor(&self) -> Inspection<ProcessorInfo> {
            Inspection::unavailable()
        }
        fn inspect_memory(&self) -> Inspection<MemoryInfo> {
            Inspection::available(MemoryInfo {
                total_bytes: 100,
                used_bytes: 25,
                available_bytes: 75,
                swap_total_bytes: 0,
                swap_used_bytes: 0,
            })
        }
        fn inspect_storage(&self) -> Inspection<Vec<StorageDevice>> {
            Inspection::failed("blocked")
        }
        fn inspect_network(&self) -> Inspection<Vec<NetworkInterface>> {
            Inspection::unavailable()
        }
        fn inspect_thermal(&self) -> Inspection<Vec<ThermalSensor>> {
            Inspection::unavailable()
        }
        fn inspect_firmware(&self) -> Inspection<FirmwareInfo> {
            Inspection::unavailable()
        }
        fn inspect_graphics(&self) -> Inspection<Vec<GraphicsDevice>> {
            Inspection::unavailable()
        }
        fn inspect_power(&self) -> Inspection<Vec<PowerSupply>> {
            Inspection::unavailable()
        }
    }
    #[test]
    fn partial_failures_do_not_erase_successful_categories() {
        let report = InspectMachine::new(PartialInspector).execute();
        assert!(matches!(report.identity, Inspection::Available(_)));
        assert!(matches!(report.memory, Inspection::Available(_)));
        assert!(matches!(report.storage, Inspection::Failed(_)));
        assert_eq!(report.processor, Inspection::Unavailable);
    }
}
