#![forbid(unsafe_code)]
use std::{fs, path::Path};
use sysinfo::{CpuRefreshKind, Disks, Networks, System};
use zashikura_application::{MachineInspector, SystemInspectionError, SystemInspector};
use zashikura_core::{
    FirmwareInfo, GraphicsDevice, Inspection, LogicalProcessor, MachineIdentity, MemoryInfo,
    NetworkInterface, PowerSupply, ProcessorInfo, StorageDevice, ThermalSensor,
};
use zashikura_core::{MemorySnapshot, SystemSnapshot};

#[derive(Clone, Copy, Debug, Default)]
pub struct SysinfoMachineInspector;

#[derive(Clone, Copy, Debug, Default)]
pub struct SysinfoSystemInspector;

impl SystemInspector for SysinfoSystemInspector {
    fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError> {
        SysinfoMachineInspector.inspect_system()
    }
}

impl SystemInspector for SysinfoMachineInspector {
    fn inspect_system(&self) -> Result<SystemSnapshot, SystemInspectionError> {
        let mut system = System::new();
        system.refresh_cpu_list(CpuRefreshKind::everything());
        system.refresh_memory();
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
impl MachineInspector for SysinfoMachineInspector {
    fn inspect_identity(&self) -> Inspection<MachineIdentity> {
        Inspection::available(MachineIdentity {
            hostname: System::host_name(),
            operating_system: System::name(),
            os_version: System::os_version(),
            kernel_version: System::kernel_version(),
            architecture: Some(System::cpu_arch()),
            uptime_seconds: Some(System::uptime()),
        })
    }
    fn inspect_processor(&self) -> Inspection<ProcessorInfo> {
        let mut system = System::new();
        system.refresh_cpu_list(CpuRefreshKind::everything());
        let cpus = system.cpus();
        let Some(first) = cpus.first() else {
            return Inspection::unavailable();
        };
        Inspection::available(ProcessorInfo {
            vendor: non_empty(first.vendor_id()),
            brand: non_empty(first.brand()),
            logical_processors: cpus.len(),
            physical_cores: System::physical_core_count(),
            processors: cpus
                .iter()
                .map(|cpu| LogicalProcessor {
                    name: cpu.name().to_owned(),
                    frequency_mhz: (cpu.frequency() > 0).then_some(cpu.frequency()),
                })
                .collect(),
        })
    }
    fn inspect_memory(&self) -> Inspection<MemoryInfo> {
        let mut system = System::new();
        system.refresh_memory();
        let total = system.total_memory();
        if total == 0 {
            return Inspection::unavailable();
        }
        Inspection::available(MemoryInfo {
            total_bytes: total,
            used_bytes: system.used_memory(),
            available_bytes: system.available_memory(),
            swap_total_bytes: system.total_swap(),
            swap_used_bytes: system.used_swap(),
        })
    }
    fn inspect_storage(&self) -> Inspection<Vec<StorageDevice>> {
        let disks = Disks::new_with_refreshed_list();
        Inspection::available(
            disks
                .iter()
                .map(|disk| StorageDevice {
                    name: disk.name().to_string_lossy().into_owned(),
                    file_system: disk.file_system().to_string_lossy().into_owned(),
                    mount_point: disk.mount_point().display().to_string(),
                    total_bytes: disk.total_space(),
                    available_bytes: disk.available_space(),
                    removable: disk.is_removable(),
                    read_only: disk.is_read_only(),
                })
                .collect(),
        )
    }
    fn inspect_network(&self) -> Inspection<Vec<NetworkInterface>> {
        let networks = Networks::new_with_refreshed_list();
        Inspection::available(
            networks
                .iter()
                .map(|(name, data)| NetworkInterface {
                    name: name.clone(),
                    received_bytes: data.total_received(),
                    transmitted_bytes: data.total_transmitted(),
                    received_packets: data.total_packets_received(),
                    transmitted_packets: data.total_packets_transmitted(),
                })
                .collect(),
        )
    }
    fn inspect_thermal(&self) -> Inspection<Vec<ThermalSensor>> {
        #[cfg(target_os = "linux")]
        {
            let entries = match read_dir_entries(Path::new("/sys/class/thermal")) {
                Ok(Some(entries)) => entries,
                Ok(None) => return Inspection::unavailable(),
                Err(error) => return Inspection::failed(error),
            };
            let mut sensors = Vec::new();
            for entry in entries {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.starts_with("thermal_zone") {
                    continue;
                }
                let path = entry.path();
                let temperature =
                    read_signed_number(&path.join("temp")).map(|value| value as f32 / 1000.0);
                if temperature.is_some() {
                    let (maximum_celsius, critical_celsius) = read_thermal_thresholds(&path);
                    sensors.push(ThermalSensor {
                        label: read_attribute(&path.join("type")).unwrap_or(name),
                        temperature_celsius: temperature,
                        maximum_celsius,
                        critical_celsius,
                    });
                }
            }
            if sensors.is_empty() {
                Inspection::unavailable()
            } else {
                Inspection::available(sensors)
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Inspection::unavailable()
        }
    }
    fn inspect_firmware(&self) -> Inspection<FirmwareInfo> {
        #[cfg(target_os = "linux")]
        {
            let root = Path::new("/sys/class/dmi/id");
            let info = FirmwareInfo {
                system_vendor: read_attribute(&root.join("sys_vendor")),
                product_name: read_attribute(&root.join("product_name")),
                product_version: read_attribute(&root.join("product_version")),
                board_vendor: read_attribute(&root.join("board_vendor")),
                board_name: read_attribute(&root.join("board_name")),
                board_version: read_attribute(&root.join("board_version")),
                bios_vendor: read_attribute(&root.join("bios_vendor")),
                bios_version: read_attribute(&root.join("bios_version")),
                bios_date: read_attribute(&root.join("bios_date")),
            };
            let fields = [
                &info.system_vendor,
                &info.product_name,
                &info.product_version,
                &info.board_vendor,
                &info.board_name,
                &info.board_version,
                &info.bios_vendor,
                &info.bios_version,
                &info.bios_date,
            ];
            if fields.iter().all(|field| field.is_none()) {
                Inspection::unavailable()
            } else {
                Inspection::available(info)
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Inspection::unavailable()
        }
    }
    fn inspect_graphics(&self) -> Inspection<Vec<GraphicsDevice>> {
        #[cfg(target_os = "linux")]
        {
            let entries = match read_dir_entries(Path::new("/sys/class/drm")) {
                Ok(Some(entries)) => entries,
                Ok(None) => return Inspection::unavailable(),
                Err(error) => return Inspection::failed(error),
            };
            let mut devices = Vec::new();
            for entry in entries {
                let node = entry.file_name().to_string_lossy().into_owned();
                let Some(index) = node.strip_prefix("card") else {
                    continue;
                };
                if index.is_empty() || !index.chars().all(|c| c.is_ascii_digit()) {
                    continue;
                }
                let device = entry.path().join("device");
                if !device.exists() {
                    continue;
                }
                let driver = fs::read_link(device.join("driver")).ok().and_then(|path| {
                    path.file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                });
                devices.push(GraphicsDevice {
                    device_node: node,
                    vendor_id: read_attribute(&device.join("vendor")),
                    device_id: read_attribute(&device.join("device")),
                    driver,
                });
            }
            if devices.is_empty() {
                Inspection::unavailable()
            } else {
                Inspection::available(devices)
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Inspection::unavailable()
        }
    }
    fn inspect_power(&self) -> Inspection<Vec<PowerSupply>> {
        #[cfg(target_os = "linux")]
        {
            let entries = match read_dir_entries(Path::new("/sys/class/power_supply")) {
                Ok(Some(entries)) => entries,
                Ok(None) => return Inspection::unavailable(),
                Err(error) => return Inspection::failed(error),
            };
            let supplies = entries
                .into_iter()
                .map(|entry| {
                    let path = entry.path();
                    let number = |key: &str| read_number(&path.join(key));
                    PowerSupply {
                        name: entry.file_name().to_string_lossy().into_owned(),
                        kind: read_attribute(&path.join("type")),
                        model: read_attribute(&path.join("model_name")),
                        manufacturer: read_attribute(&path.join("manufacturer")),
                        status: read_attribute(&path.join("status")),
                        capacity_percent: number("capacity")
                            .and_then(|v| u8::try_from(v).ok())
                            .filter(|v| *v <= 100),
                        voltage_microvolts: number("voltage_now"),
                        current_microamps: number("current_now"),
                        power_microwatts: number("power_now"),
                        energy_now_microwatt_hours: number("energy_now"),
                        energy_full_microwatt_hours: number("energy_full"),
                    }
                })
                .collect::<Vec<_>>();
            if supplies.is_empty() {
                Inspection::unavailable()
            } else {
                Inspection::available(supplies)
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Inspection::unavailable()
        }
    }
}
fn non_empty(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}
#[cfg(target_os = "linux")]
fn read_attribute(path: &Path) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .and_then(|value| non_empty(&value))
}
#[cfg(target_os = "linux")]
fn read_number(path: &Path) -> Option<u64> {
    read_attribute(path)?.parse().ok()
}

#[cfg(target_os = "linux")]
fn read_signed_number(path: &Path) -> Option<i64> {
    parse_signed_number(&read_attribute(path)?)
}

fn parse_signed_number(value: &str) -> Option<i64> {
    value.trim().parse().ok()
}

#[cfg(target_os = "linux")]
fn read_dir_entries(path: &Path) -> Result<Option<Vec<fs::DirEntry>>, String> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .collect::<Result<Vec<_>, _>>()
            .map(Some)
            .map_err(|error| format!("{}: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

#[cfg(target_os = "linux")]
fn read_thermal_thresholds(path: &Path) -> (Option<f32>, Option<f32>) {
    let Ok(Some(entries)) = read_dir_entries(path) else {
        return (None, None);
    };
    let mut maximum: Option<i64> = None;
    let mut critical: Option<i64> = None;
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(index) = name
            .strip_prefix("trip_point_")
            .and_then(|value| value.strip_suffix("_temp"))
        else {
            continue;
        };
        let Some(temperature) = read_signed_number(&entry.path()) else {
            continue;
        };
        maximum = Some(maximum.map_or(temperature, |value| value.max(temperature)));
        let kind = path.join(format!("trip_point_{index}_type"));
        if read_attribute(&kind).is_some_and(|value| value.eq_ignore_ascii_case("critical")) {
            critical = Some(critical.map_or(temperature, |value| value.max(temperature)));
        }
    }
    (
        maximum.map(|value| value as f32 / 1000.0),
        critical.map(|value| value as f32 / 1000.0),
    )
}

#[cfg(test)]
mod tests {
    use super::{SysinfoMachineInspector, non_empty};
    use zashikura_application::MachineInspector;
    use zashikura_core::Inspection;
    #[test]
    fn blank_attributes_are_missing() {
        assert_eq!(non_empty("  \n"), None);
        assert_eq!(non_empty(" value \n").as_deref(), Some("value"));
    }
    #[test]
    fn signed_thermal_values_preserve_negative_temperatures() {
        assert_eq!(super::parse_signed_number(" -1250\n"), Some(-1250));
        assert_eq!(super::parse_signed_number("invalid"), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn missing_sysfs_category_is_unavailable_not_failed() {
        use std::path::Path;
        assert!(matches!(
            super::read_dir_entries(Path::new("/definitely/not/a/zashikura/path")),
            Ok(None)
        ));
    }

    #[test]
    fn legacy_system_inspection_remains_available() {
        use super::SysinfoSystemInspector;
        use zashikura_application::SystemInspector;

        let snapshot = SysinfoSystemInspector.inspect_system().unwrap();
        assert!(snapshot.logical_cpu_count > 0);
        assert!(snapshot.memory.total_bytes > 0);
        assert!(snapshot.memory.used_bytes <= snapshot.memory.total_bytes);
    }

    #[test]
    fn independent_categories_can_be_collected() {
        let inspector = SysinfoMachineInspector;
        assert!(matches!(
            inspector.inspect_identity(),
            Inspection::Available(_)
        ));
        let _ = inspector.inspect_storage();
        let _ = inspector.inspect_network();
        let _ = inspector.inspect_thermal();
        let _ = inspector.inspect_firmware();
        let _ = inspector.inspect_graphics();
        let _ = inspector.inspect_power();
    }
}
