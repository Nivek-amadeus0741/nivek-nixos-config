use crate::bus::EventBus;
use anyhow::Result;
use nivek_wasm_abi::SystemEvent;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};

pub fn read_battery_status() -> Option<(u8, bool)> {
    let base = Path::new("/sys/class/power_supply");
    if !base.exists() {
        return None;
    }

    if let Ok(entries) = fs::read_dir(base) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with("BAT") {
                let capacity_path = path.join("capacity");
                let status_path = path.join("status");

                if let Ok(cap_str) = fs::read_to_string(&capacity_path) {
                    if let Ok(percentage) = cap_str.trim().parse::<u8>() {
                        let status_str = fs::read_to_string(&status_path).unwrap_or_default();
                        let on_battery = status_str.trim().eq_ignore_ascii_case("Discharging");
                        return Some((percentage, on_battery));
                    }
                }
            }
        }
    }

    None
}

pub fn set_cpu_governor(governor: &str) -> Result<String> {
    debug!("Intentando aplicar gobernador de CPU: '{}'", governor);

    // 1. Intentamos powerprofilesctl si está disponible
    let profile = if governor == "performance" {
        "performance"
    } else if governor == "powersave" {
        "power-saver"
    } else {
        "balanced"
    };

    if let Ok(output) = std::process::Command::new("powerprofilesctl")
        .args(["set", profile])
        .output()
    {
        if output.status.success() {
            return Ok(format!(
                "Gobernador establecido a '{}' (powerprofilesctl)",
                profile
            ));
        }
    }

    // 2. Fallback: sysfs scaling_governor directo
    let cpu_base = Path::new("/sys/devices/system/cpu");
    if cpu_base.exists() {
        if let Ok(entries) = fs::read_dir(cpu_base) {
            let mut applied_count = 0;
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit()) {
                    let gov_file = path.join("cpufreq/scaling_governor");
                    if gov_file.exists() && fs::write(&gov_file, governor.as_bytes()).is_ok() {
                        applied_count += 1;
                    }
                }
            }
            if applied_count > 0 {
                return Ok(format!(
                    "Gobernador '{}' aplicado a {} núcleos vía sysfs",
                    governor, applied_count
                ));
            }
        }
    }

    Ok(format!("Perfil de CPU '{}' registrado", governor))
}

pub async fn run_power_monitor(bus: Arc<EventBus>) {
    info!("Iniciando monitor de telemetría de batería y energía...");

    let mut last_percentage: Option<u8> = None;
    let mut last_on_battery: Option<bool> = None;

    loop {
        if let Some((percentage, on_battery)) = read_battery_status() {
            let has_changed = match (last_percentage, last_on_battery) {
                (Some(p), Some(b)) => p != percentage || b != on_battery,
                _ => true,
            };

            if has_changed {
                debug!(
                    "Telemetría de batería actualizada: {}% (en batería: {})",
                    percentage, on_battery
                );
                bus.publish(SystemEvent::PowerStateChanged {
                    percentage,
                    on_battery,
                });
                last_percentage = Some(percentage);
                last_on_battery = Some(on_battery);
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
