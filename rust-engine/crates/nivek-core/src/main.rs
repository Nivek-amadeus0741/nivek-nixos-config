mod audio;
mod bus;
mod collectors;
mod hardware;
mod morpher;
mod server;
mod state;
mod wasm;

use anyhow::Result;
use bus::EventBus;
use state::DaemonState;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Iniciando nivek-core daemon v{}", env!("CARGO_PKG_VERSION"));
    info!("Arquitectura: NixOS Reactive OS Engine (Rust Tokio Runtime)");

    let socket_dir = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));
    let socket_path = socket_dir.join("nivek-engine.sock");

    if socket_path.exists() {
        let _ = std::fs::remove_file(&socket_path);
    }

    let listener = UnixListener::bind(&socket_path)?;
    info!(
        "Servidor IPC escuchando en socket UNIX: {}",
        socket_path.display()
    );

    let bus = Arc::new(EventBus::new(2048));
    let state = Arc::new(DaemonState::new());

    // 1. Colector de Wayland IPC (Niri)
    let niri_bus = Arc::clone(&bus);
    tokio::spawn(async move {
        collectors::niri::run_niri_collector(niri_bus).await;
    });

    // 2. Monitor de Telemetría de Batería y Energía
    let power_bus = Arc::clone(&bus);
    tokio::spawn(async move {
        hardware::power::run_power_monitor(power_bus).await;
    });

    // 3. Motor de políticas de Morfismo Contextual
    let morpher_bus = Arc::clone(&bus);
    let morpher_rx = bus.subscribe();
    tokio::spawn(async move {
        morpher::run_context_morpher(morpher_rx, morpher_bus).await;
    });

    // 4. Gestor de plugins WebAssembly (WASM Hot-Reload)
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".config")))
        .unwrap_or_else(|_| PathBuf::from("/tmp"));
    let rules_dir = config_dir.join("nivek").join("rules");

    let wasm_bus = Arc::clone(&bus);
    let wasm_rx = bus.subscribe();
    tokio::spawn(async move {
        wasm::manager::run_wasm_manager(rules_dir, wasm_bus, wasm_rx).await;
    });

    // Publicamos evento de ciclo de vida
    bus.publish(nivek_wasm_abi::SystemEvent::Custom {
        topic: "system.lifecycle".to_string(),
        message: "nivek-core inicializado con soporte Niri, Hardware, PipeWire y WASM".to_string(),
    });

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let bus_clone = Arc::clone(&bus);
                let state_clone = Arc::clone(&state);
                tokio::spawn(async move {
                    if let Err(e) = server::handle_client(stream, bus_clone, state_clone).await {
                        error!("Sesión de cliente finalizada: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Error aceptando conexión en socket UNIX: {}", e);
            }
        }
    }
}
