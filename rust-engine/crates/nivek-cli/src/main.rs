use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use nivek_wasm_abi::{IpcRequest, IpcResponse, SystemAction, SystemEvent};
use std::path::PathBuf;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

#[derive(Parser)]
#[command(name = "nivekctl")]
#[command(author = "Nivek")]
#[command(version = "0.1.0")]
#[command(about = "CLI de control, telemetría y eventos para el motor reactivo de NixOS")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Comprueba la conexión y mide la latencia de respuesta (RTT)
    Ping,

    /// Muestra el panel de métricas y estado del demonio
    Status,

    /// Escucha el flujo de eventos del sistema operativo en tiempo real
    Monitor,

    /// Emite un evento manual al bus del sistema
    Emit {
        #[command(subcommand)]
        event: EmitCommand,
    },

    /// Ejecuta una acción de control sobre el sistema
    Action {
        #[command(subcommand)]
        action: ActionCommand,
    },
}

#[derive(Subcommand)]
enum EmitCommand {
    /// Emite un evento de cambio de ventana activa
    Window {
        #[arg(short, long)]
        app_id: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long, default_value_t = 1)]
        workspace: u32,
    },
    /// Emite un evento de cambio de workspace
    Workspace {
        #[arg(short, long)]
        num: u32,
    },
    /// Emite un evento de nivel de batería o alimentación
    Power {
        #[arg(short, long)]
        percentage: u8,
        #[arg(short, long, default_value_t = false)]
        on_battery: bool,
    },
    /// Solicita un cambio de contexto (dev, media, gaming, etc.)
    Context {
        #[arg(short, long)]
        name: String,
    },
    /// Emite un evento libre o genérico
    Custom {
        #[arg(short, long)]
        topic: String,
        #[arg(short, long)]
        message: String,
    },
}

#[derive(Subcommand)]
enum ActionCommand {
    /// Cambia el gobernador de frecuencia de la CPU
    Governor {
        #[arg(short, long)]
        name: String,
    },
    /// Ajusta el brillo de pantalla vía DDC/CI
    Brightness {
        #[arg(short, long, default_value_t = 1)]
        display: u32,
        #[arg(short, long)]
        value: u8,
    },
    /// Dispara una notificación de escritorio
    Notify {
        #[arg(short, long)]
        summary: String,
        #[arg(short, long)]
        body: String,
        #[arg(short, long, default_value = "normal")]
        urgency: String,
    },
    /// Cambia el perfil de audio en PipeWire
    AudioProfile {
        #[arg(short, long)]
        profile: String,
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
            "❌ Error: El demonio nivek-core no está en ejecución (socket no encontrado en {})",
            socket_path.display()
        );
        std::process::exit(1);
    }

    let stream = UnixStream::connect(&socket_path)
        .await
        .with_context(|| format!("No se pudo conectar al socket {}", socket_path.display()))?;

    let (reader, mut writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);

    match cli.command {
        Commands::Ping => {
            let start = Instant::now();
            send_request(&mut writer, &IpcRequest::Ping).await?;
            let res = read_response(&mut buf_reader).await?;
            let rtt = start.elapsed();

            if let IpcResponse::Pong {
                version,
                uptime_secs,
            } = res
            {
                println!(
                    "🟢 nivek-core está activo (v{}) | RTT: {:.2?} | Uptime: {}s",
                    version, rtt, uptime_secs
                );
            } else {
                println!("Respuesta inesperada: {:?}", res);
            }
        }
        Commands::Status => {
            send_request(&mut writer, &IpcRequest::GetStatus).await?;
            let res = read_response(&mut buf_reader).await?;

            if let IpcResponse::Status(status) = res {
                let mb = (status.memory_rss_bytes as f64) / (1024.0 * 1024.0);
                let mins = status.uptime_secs / 60;
                let secs = status.uptime_secs % 60;

                println!("┌────────────────────────────────────────────────────────┐");
                println!("│  Nivek Reactive OS Engine Status                       │");
                println!("├────────────────────────────────────────────────────────┤");
                println!("│  Versión del Demonio:    {:<30}│", status.version);
                println!(
                    "│  Tiempo Activo (Uptime): {:<30}│",
                    format!("{}m {}s", mins, secs)
                );
                println!(
                    "│  Eventos Procesados:     {:<30}│",
                    status.total_events_dispatched
                );
                println!(
                    "│  Suscriptores Activos:   {:<30}│",
                    status.active_subscribers
                );
                println!(
                    "│  Consumo de Memoria RSS: {:<30}│",
                    format!("{:.2} MB", mb)
                );
                println!("└────────────────────────────────────────────────────────┘");
            } else {
                println!("Respuesta inesperada: {:?}", res);
            }
        }
        Commands::Emit { event } => {
            let sys_event = match event {
                EmitCommand::Window {
                    app_id,
                    title,
                    workspace,
                } => SystemEvent::WindowFocusChanged {
                    app_id,
                    title,
                    workspace,
                },
                EmitCommand::Workspace { num } => {
                    SystemEvent::WorkspaceActivated { workspace: num }
                }
                EmitCommand::Power {
                    percentage,
                    on_battery,
                } => SystemEvent::PowerStateChanged {
                    percentage,
                    on_battery,
                },
                EmitCommand::Context { name } => SystemEvent::ContextRequested { context: name },
                EmitCommand::Custom { topic, message } => SystemEvent::Custom { topic, message },
            };

            send_request(&mut writer, &IpcRequest::EmitEvent(sys_event)).await?;
            let res = read_response(&mut buf_reader).await?;
            if let IpcResponse::Ack { success, message } = res {
                if success {
                    println!("✨ {}", message);
                } else {
                    eprintln!("⚠️ Fallo: {}", message);
                }
            }
        }
        Commands::Action { action } => {
            let sys_action = match action {
                ActionCommand::Governor { name } => SystemAction::SetCpuGovernor { governor: name },
                ActionCommand::Brightness { display, value } => {
                    SystemAction::SetDisplayBrightness {
                        display,
                        brightness: value,
                    }
                }
                ActionCommand::Notify {
                    summary,
                    body,
                    urgency,
                } => SystemAction::Notify {
                    summary,
                    body,
                    urgency,
                },
                ActionCommand::AudioProfile { profile } => {
                    SystemAction::SetAudioProfile { profile }
                }
            };

            send_request(&mut writer, &IpcRequest::TriggerAction(sys_action)).await?;
            let res = read_response(&mut buf_reader).await?;
            if let IpcResponse::Ack { success, message } = res {
                if success {
                    println!("⚡ {}", message);
                } else {
                    eprintln!("⚠️ Fallo: {}", message);
                }
            }
        }
        Commands::Monitor => {
            println!("📡 Conectado al bus de eventos de nivek-core. Escuchando en vivo (Ctrl+C para salir)...");
            send_request(&mut writer, &IpcRequest::SubscribeEvents).await?;

            let first_res = read_response(&mut buf_reader).await?;
            if let IpcResponse::Ack { message, .. } = first_res {
                println!("ℹ️  {}", message);
            }

            loop {
                match read_response(&mut buf_reader).await {
                    Ok(IpcResponse::EventNotification(event)) => {
                        format_and_print_event(&event);
                    }
                    Ok(other) => {
                        println!("Mensaje recibido: {:?}", other);
                    }
                    Err(e) => {
                        eprintln!("Desconectado del servidor: {}", e);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn send_request(
    writer: &mut tokio::net::unix::OwnedWriteHalf,
    request: &IpcRequest,
) -> Result<()> {
    let mut json = serde_json::to_string(request)?;
    json.push('\n');
    writer.write_all(json.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

async fn read_response(
    reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
) -> Result<IpcResponse> {
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    let res: IpcResponse = serde_json::from_str(line.trim())
        .with_context(|| format!("No se pudo parsear respuesta: {}", line))?;
    Ok(res)
}

fn format_and_print_event(event: &SystemEvent) {
    match event {
        SystemEvent::WindowFocusChanged {
            app_id,
            title,
            workspace,
        } => {
            println!(
                "🪟 [VENTANA] Workspace {}: '{}' ({})",
                workspace, title, app_id
            );
        }
        SystemEvent::WorkspaceActivated { workspace } => {
            println!("📌 [WORKSPACE] Cambiado a espacio {}", workspace);
        }
        SystemEvent::PowerStateChanged {
            on_battery,
            percentage,
        } => {
            let mode = if *on_battery {
                "Batería"
            } else {
                "AC Conectado"
            };
            println!("🔋 [ENERGÍA] {}% ({})", percentage, mode);
        }
        SystemEvent::ContextRequested { context } => {
            println!("🎯 [CONTEXTO] Solicitado modo: {}", context);
        }
        SystemEvent::Custom { topic, message } => {
            println!("📢 [{}] {}", topic, message);
        }
    }
}
