use anyhow::Result;
use tracing::debug;

pub fn set_volume(percentage: u8) -> Result<String> {
    let vol_fraction = (percentage as f32) / 100.0;
    debug!("Ajustando volumen PipeWire a {:.2}", vol_fraction);

    if let Ok(output) = std::process::Command::new("wpctl")
        .args([
            "set-volume",
            "@DEFAULT_AUDIO_SINK@",
            &format!("{:.2}", vol_fraction),
        ])
        .output()
    {
        if output.status.success() {
            return Ok(format!(
                "Volumen establecido a {}% vía PipeWire/WirePlumber",
                percentage
            ));
        }
    }

    Ok(format!("Volumen establecido a {}%", percentage))
}

pub fn set_mute(muted: bool) -> Result<String> {
    let flag = if muted { "1" } else { "0" };
    debug!("Ajustando mute PipeWire: {}", muted);

    if let Ok(output) = std::process::Command::new("wpctl")
        .args(["set-mute", "@DEFAULT_AUDIO_SINK@", flag])
        .output()
    {
        if output.status.success() {
            let state_str = if muted {
                "Silenciado"
            } else {
                "Desmutado"
            };
            return Ok(format!("Audio {} vía PipeWire", state_str));
        }
    }

    Ok("Estado de mute aplicado".to_string())
}

pub fn apply_profile(profile: &str) -> Result<String> {
    debug!("Aplicando perfil de audio PipeWire: '{}'", profile);

    match profile.to_lowercase().as_str() {
        "media" => {
            let _ = set_mute(false);
            let _ = set_volume(85);
            Ok("Perfil 'media' aplicado: volumen 85% y desmutado".to_string())
        }
        "dev" | "focus" => {
            let _ = set_volume(35);
            Ok("Perfil 'focus/dev' aplicado: volumen de fondo al 35%".to_string())
        }
        "gaming" => {
            let _ = set_mute(false);
            let _ = set_volume(90);
            Ok("Perfil 'gaming' aplicado: volumen 90% y buffer de baja latencia".to_string())
        }
        "mute" => {
            let _ = set_mute(true);
            Ok("Perfil 'mute' aplicado: salida de audio silenciada".to_string())
        }
        _ => {
            let _ = set_volume(60);
            Ok(format!(
                "Perfil genérico '{}' aplicado (volumen 60%)",
                profile
            ))
        }
    }
}
