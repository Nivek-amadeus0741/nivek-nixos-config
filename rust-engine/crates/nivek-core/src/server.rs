use crate::bus::EventBus;
use crate::state::DaemonState;
use anyhow::Result;
use nivek_wasm_abi::{DaemonStatus, IpcRequest, IpcResponse, SystemAction};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{info, warn};

pub async fn handle_client(
    stream: UnixStream,
    bus: Arc<EventBus>,
    state: Arc<DaemonState>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    while buf_reader.read_line(&mut line).await? > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        let request: Result<IpcRequest, _> = serde_json::from_str(trimmed);
        match request {
            Ok(IpcRequest::Ping) => {
                let res = IpcResponse::Pong {
                    version: state.version.to_string(),
                    uptime_secs: state.uptime_secs(),
                };
                send_response(&mut writer, &res).await?;
            }
            Ok(IpcRequest::GetStatus) => {
                let status = DaemonStatus {
                    version: state.version.to_string(),
                    uptime_secs: state.uptime_secs(),
                    total_events_dispatched: bus.total_events(),
                    active_subscribers: bus.subscriber_count(),
                    memory_rss_bytes: state.memory_rss_bytes(),
                };
                send_response(&mut writer, &IpcResponse::Status(status)).await?;
            }
            Ok(IpcRequest::EmitEvent(event)) => {
                info!("Inyectando evento desde IPC: {:?}", event);
                bus.publish(event);
                let res = IpcResponse::Ack {
                    success: true,
                    message: "Evento emitido al bus del sistema".to_string(),
                };
                send_response(&mut writer, &res).await?;
            }
            Ok(IpcRequest::TriggerAction(action)) => {
                info!("Acción recibida: {:?}", action);
                let msg = execute_action(&action);
                let res = IpcResponse::Ack {
                    success: true,
                    message: msg,
                };
                send_response(&mut writer, &res).await?;
            }
            Ok(IpcRequest::SubscribeEvents) => {
                info!("Cliente suscrito a stream de eventos");
                let mut receiver = bus.subscribe();
                let ack = IpcResponse::Ack {
                    success: true,
                    message: "Stream de eventos activado".to_string(),
                };
                send_response(&mut writer, &ack).await?;

                loop {
                    tokio::select! {
                        event_res = receiver.recv() => {
                            match event_res {
                                Ok(event) => {
                                    let notification = IpcResponse::EventNotification((*event).clone());
                                    if send_response(&mut writer, &notification).await.is_err() {
                                        break;
                                    }
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
                                    warn!("Suscriptor retrasado, se perdieron {} eventos", missed);
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                    break;
                                }
                            }
                        }
                    }
                }
                info!("Cliente de stream desconectado");
                return Ok(());
            }
            Err(e) => {
                let err_res = IpcResponse::Error {
                    message: format!("Petición inválida: {}", e),
                };
                send_response(&mut writer, &err_res).await?;
            }
        }

        line.clear();
    }

    Ok(())
}

async fn send_response(
    writer: &mut tokio::net::unix::OwnedWriteHalf,
    response: &IpcResponse,
) -> Result<()> {
    let mut json = serde_json::to_string(response)?;
    json.push('\n');
    writer.write_all(json.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

fn execute_action(action: &SystemAction) -> String {
    match action {
        SystemAction::SetDisplayBrightness {
            display,
            brightness,
        } => crate::hardware::display::set_display_brightness(*display, *brightness)
            .unwrap_or_else(|e| format!("Error ajustando brillo: {}", e)),
        SystemAction::SetCpuGovernor { governor } => crate::hardware::power::set_cpu_governor(governor)
            .unwrap_or_else(|e| format!("Error en CPU governor: {}", e)),
        SystemAction::Notify {
            summary,
            body,
            urgency,
        } => {
            let _ = std::process::Command::new("notify-send")
                .args(["-u", urgency, summary, body])
                .spawn();
            format!("Notificación enviada: [{}] {}: {}", urgency, summary, body)
        }
        SystemAction::SetAudioProfile { profile } => crate::audio::pipewire::apply_profile(profile)
            .unwrap_or_else(|e| format!("Error en perfil de audio: {}", e)),
    }
}
