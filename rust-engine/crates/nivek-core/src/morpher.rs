use crate::bus::EventBus;
use nivek_wasm_abi::SystemEvent;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

pub async fn run_context_morpher(mut receiver: broadcast::Receiver<Arc<SystemEvent>>, bus: Arc<EventBus>) {
    info!("Iniciando motor de políticas reactivas: Context Morpher...");

    let mut current_context = "balanced".to_string();

    while let Ok(event) = receiver.recv().await {
        if let SystemEvent::WindowFocusChanged { app_id, title, .. } = &*event {
            let target_context = determine_context(app_id, title);

            if target_context != current_context {
                info!(
                    "🎯 [CONTEXT MORPHER] Transición de contexto: '{}' ──► '{}' (App: '{}', Título: '{}')",
                    current_context, target_context, app_id, title
                );

                apply_context_policy(&target_context, &bus);
                current_context = target_context.clone();

                // Notificar al bus global sobre el cambio de contexto
                bus.publish(SystemEvent::ContextRequested {
                    context: target_context,
                });
            }
        }
    }
}

fn determine_context(app_id: &str, title: &str) -> String {
    let app_lower = app_id.to_lowercase();
    let title_lower = title.to_lowercase();

    // Detección de herramientas de desarrollo
    if app_lower.contains("neovim")
        || app_lower.contains("nvim")
        || app_lower.contains("foot")
        || app_lower.contains("alacritty")
        || app_lower.contains("wezterm")
        || app_lower.contains("kitty")
        || app_lower.contains("code")
        || app_lower.contains("zed")
        || title_lower.contains(".rs")
        || title_lower.contains(".nix")
        || title_lower.contains(".cpp")
    {
        return "dev".to_string();
    }

    // Detección de multimedia
    if app_lower.contains("mpv")
        || app_lower.contains("spotify")
        || app_lower.contains("vlc")
        || app_lower.contains("freetube")
        || title_lower.contains("youtube")
        || title_lower.contains("netflix")
    {
        return "media".to_string();
    }

    // Detección de videojuegos y 3D
    if app_lower.contains("steam")
        || app_lower.contains("lutris")
        || app_lower.contains("heroic")
        || app_lower.contains("retroarch")
        || app_lower.contains("gamescope")
    {
        return "gaming".to_string();
    }

    "balanced".to_string()
}

fn apply_context_policy(context: &str, bus: &EventBus) {
    match context {
        "dev" => {
            bus.publish(SystemEvent::Custom {
                topic: "policy.governor".to_string(),
                message: "CPU Governor set to 'performance' for development workflow".to_string(),
            });
        }
        "media" => {
            bus.publish(SystemEvent::Custom {
                topic: "policy.audio".to_string(),
                message: "PipeWire dynamic range compression activated for media".to_string(),
            });
        }
        "gaming" => {
            bus.publish(SystemEvent::Custom {
                topic: "policy.gpu".to_string(),
                message: "Maximum GPU clocks & VRR priority engaged for gaming".to_string(),
            });
        }
        _ => {
            bus.publish(SystemEvent::Custom {
                topic: "policy.power".to_string(),
                message: "System governor reverted to balanced/schedutil".to_string(),
            });
        }
    }
}
