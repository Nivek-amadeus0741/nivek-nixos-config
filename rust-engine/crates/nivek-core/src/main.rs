mod bus;
mod server;
mod state;

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

    let bus = Arc::new(EventBus::new(1024));
    let state = Arc::new(DaemonState::new());

    // Publicamos un evento inicial de arranque del sistema
    bus.publish(nivek_wasm_abi::SystemEvent::Custom {
        topic: "system.lifecycle".to_string(),
        message: "nivek-core daemon inicializado con éxito".to_string(),
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
