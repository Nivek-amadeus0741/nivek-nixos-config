use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[derive(Parser)]
#[command(name = "nivekctl")]
#[command(author = "Nivek")]
#[command(version = "0.1.0")]
#[command(about = "CLI de control y telemetría para el motor reactivo de NixOS")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Envía un ping al demonio nivek-core y verifica estado
    Ping,
    /// Consulta el estado actual de los subsistemas y eventos
    Status,
    /// Envía un comando en crudo al demonio
    Send {
        #[arg(short, long)]
        message: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let socket_dir = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));
    let socket_path = socket_dir.join("nivek-engine.sock");

    if !socket_path.exists() {
        eprintln!(
            "Error: El demonio nivek-core no está en ejecución (socket no encontrado en {})",
            socket_path.display()
        );
        std::process::exit(1);
    }

    let mut stream = UnixStream::connect(&socket_path)
        .await
        .with_context(|| format!("No se pudo conectar al socket {}", socket_path.display()))?;

    let payload = match cli.command {
        Commands::Ping => "ping".to_string(),
        Commands::Status => "status".to_string(),
        Commands::Send { message } => message,
    };

    stream.write_all(payload.as_bytes()).await?;

    let mut response = String::new();
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    response.push_str(&String::from_utf8_lossy(&buffer[..n]));

    println!("{}", response.trim());
    Ok(())
}
