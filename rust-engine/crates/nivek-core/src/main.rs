use anyhow::Result;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

    loop {
        match listener.accept().await {
            Ok((mut stream, _addr)) => {
                tokio::spawn(async move {
                    let mut buffer = [0u8; 1024];
                    match stream.read(&mut buffer).await {
                        Ok(n) if n > 0 => {
                            let request = String::from_utf8_lossy(&buffer[..n]);
                            info!("Petición recibida en socket: {}", request.trim());
                            let response = format!(
                                "{{\"status\":\"ok\",\"engine\":\"nivek-core\",\"received\":\"{}\"}}\n",
                                request.trim()
                            );
                            let _ = stream.write_all(response.as_bytes()).await;
                        }
                        _ => {}
                    }
                });
            }
            Err(e) => {
                error!("Error aceptando conexión en socket UNIX: {}", e);
            }
        }
    }
}
