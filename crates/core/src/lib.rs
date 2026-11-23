//! Platform-independent machine inspection models.
#![forbid(unsafe_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Inspection<T> { Available(T), Unavailable, Failed(InspectionFailure) }
impl<T> Inspection<T> {
    #[must_use] pub const fn available(value: T) -> Self { Self::Available(value) }
    #[must_use] pub const fn unavailable() -> Self { Self::Unavailable }
    #[must_use] pub fn failed(message: impl Into<String>) -> Self { Self::Failed(InspectionFailure { message: message.into() }) }
    #[must_use] pub const fn is_available(&self) -> bool { matches!(self, Self::Available(_)) }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectionFailure { pub message: String }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MachineSnapshot {
    pub identity: Inspection<MachineIdentity>, pub processor: Inspection<ProcessorInfo>,
    pub memory: Inspection<MemoryInfo>, pub storage: Inspection<Vec<StorageDevice>>,
    pub network: Inspection<Vec<NetworkInterface>>, pub thermal: Inspection<Vec<ThermalSensor>>,
    pub firmware: Inspection<FirmwareInfo>, pub graphics: Inspection<Vec<GraphicsDevice>>,
    pub power: Inspection<Vec<PowerSupply>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MachineIdentity {
    pub hostname: Option<String>, pub operating_system: Option<String>, pub os_version: Option<String>,
    pub kernel_version: Option<String>, pub architecture: Option<String>, pub uptime_seconds: Option<u64>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessorInfo {
    pub vendor: Option<String>, pub brand: Option<String>, pub logical_processors: usize,
    pub physical_cores: Option<usize>, pub processors: Vec<LogicalProcessor>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogicalProcessor { pub name: String, pub frequency_mhz: Option<u64> }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryInfo {
    pub total_bytes: u64, pub used_bytes: u64, pub available_bytes: u64,
    pub swap_total_bytes: u64, pub swap_used_bytes: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageDevice {
    pub name: String, pub file_system: String, pub mount_point: String, pub total_bytes: u64,
    pub available_bytes: u64, pub removable: bool, pub read_only: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkInterface {
    pub name: String, pub received_bytes: u64, pub transmitted_bytes: u64,
    pub received_packets: u64, pub transmitted_packets: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ThermalSensor {
    pub label: String, pub temperature_celsius: Option<f32>,
    pub maximum_celsius: Option<f32>, pub critical_celsius: Option<f32>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirmwareInfo {
    pub system_vendor: Option<String>, pub product_name: Option<String>, pub product_version: Option<String>,
    pub board_vendor: Option<String>, pub board_name: Option<String>, pub board_version: Option<String>,
    pub bios_vendor: Option<String>, pub bios_version: Option<String>, pub bios_date: Option<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicsDevice {
    pub device_node: String, pub vendor_id: Option<String>, pub device_id: Option<String>, pub driver: Option<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerSupply {
    pub name: String, pub kind: Option<String>, pub model: Option<String>, pub manufacturer: Option<String>,
    pub status: Option<String>, pub capacity_percent: Option<u8>, pub voltage_microvolts: Option<u64>,
    pub current_microamps: Option<u64>, pub power_microwatts: Option<u64>,
    pub energy_now_microwatt_hours: Option<u64>, pub energy_full_microwatt_hours: Option<u64>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemSnapshot {
    pub hostname: Option<String>, pub os_name: Option<String>, pub os_version: Option<String>,
    pub kernel_version: Option<String>, pub logical_cpu_count: usize, pub memory: MemorySnapshot,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemorySnapshot { pub total_bytes: u64, pub used_bytes: u64 }
impl SystemSnapshot {
    #[must_use] pub const fn new(hostname: Option<String>, os_name: Option<String>, os_version: Option<String>,
        kernel_version: Option<String>, logical_cpu_count: usize, memory: MemorySnapshot) -> Self {
        Self { hostname, os_name, os_version, kernel_version, logical_cpu_count, memory }
    }
}
impl MemorySnapshot {
    #[must_use] pub const fn new(total_bytes: u64, used_bytes: u64) -> Self { Self { total_bytes, used_bytes } }
}
#[cfg(test)]
mod tests {
    use super::{Inspection, InspectionFailure, MemorySnapshot, SystemSnapshot};
    #[test] fn unavailable_is_not_zero() {
        let result: Inspection<u64> = Inspection::unavailable();
        assert_eq!(result, Inspection::Unavailable);
        assert!(!result.is_available());
    }
    #[test] fn failed_category_keeps_reason() {
        let result: Inspection<u64> = Inspection::Failed(InspectionFailure { message: "denied".into() });
        assert_eq!(result, Inspection::failed("denied"));
    }
    #[test] fn legacy_snapshot_remains_constructible() {
        let value = SystemSnapshot::new(None, Some("Linux".into()), None, None, 4, MemorySnapshot::new(100, 25));
        assert_eq!(value.memory.used_bytes, 25);
    }
}
