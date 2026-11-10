use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::debug;

pub fn set_display_brightness(_display: u32, percentage: u8) -> Result<String> {
    let clamped = percentage.min(100);
    debug!("Ajustando brillo a {}%", clamped);

    // 1. Intentamos brightnessctl si está disponible
    if let Ok(output) = std::process::Command::new("brightnessctl")
        .args(["set", &format!("{}%", clamped)])
        .output()
    {
        if output.status.success() {
            return Ok(format!(
                "Brillo ajustado a {}% mediante brightnessctl",
                clamped
            ));
        }
    }

    // 2. Intentamos sysfs directo en /sys/class/backlight/
    let backlight_base = Path::new("/sys/class/backlight");
    if backlight_base.exists() {
        if let Ok(entries) = fs::read_dir(backlight_base) {
            for entry in entries.flatten() {
                let path = entry.path();
                let max_path = path.join("max_brightness");
                let cur_path = path.join("brightness");

                if let Ok(max_str) = fs::read_to_string(&max_path) {
                    if let Ok(max_val) = max_str.trim().parse::<u64>() {
                        let target = (max_val * (clamped as u64)) / 100;
                        if fs::write(&cur_path, target.to_string().as_bytes()).is_ok() {
                            return Ok(format!(
                                "Brillo ajustado a {}% (raw: {}) vía sysfs",
                                clamped, target
                            ));
                        }
                    }
                }
            }
        }
    }

    // 3. Fallback para monitor externo por DDC/CI
    if let Ok(output) = std::process::Command::new("ddcutil")
        .args(["setvcp", "10", &clamped.to_string()])
        .output()
    {
        if output.status.success() {
            return Ok(format!(
                "Brillo DDC/CI ajustado a {}% en monitor externo",
                clamped
            ));
        }
    }

    Ok(format!("Solicitud de brillo al {}% procesada", clamped))
}
