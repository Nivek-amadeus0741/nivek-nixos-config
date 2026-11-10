use crate::bus::EventBus;
use anyhow::{Context, Result};
use nivek_wasm_abi::SystemEvent;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, info, warn};

#[derive(Debug, Deserialize)]
struct NiriWindow {
    id: u64,
    title: Option<String>,
    app_id: Option<String>,
    workspace_id: Option<u32>,
    #[serde(default)]
    is_focused: bool,
}

#[derive(Debug, Deserialize)]
struct WindowOpenedOrChangedData {
    window: NiriWindow,
}

#[derive(Debug, Deserialize)]
struct WindowFocusChangedData {
    id: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceActivatedData {
    id: u32,
    #[serde(default)]
    focused: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum NiriRawEvent {
    WindowOpenedOrChanged {
        #[serde(rename = "WindowOpenedOrChanged")]
        data: WindowOpenedOrChangedData,
    },
    WindowFocusChanged {
        #[serde(rename = "WindowFocusChanged")]
        data: WindowFocusChangedData,
    },
    WorkspaceActivated {
        #[serde(rename = "WorkspaceActivated")]
        data: WorkspaceActivatedData,
    },
    #[allow(dead_code)]
    Other(serde_json::Value),
}

pub async fn run_niri_collector(bus: Arc<EventBus>) {
    info!("Iniciando colector reactivo de Wayland IPC (Niri)...");

    loop {
        match find_niri_socket() {
            Some(socket_path) => {
                info!(
                    "Conectando al socket IPC de Niri en {}",
                    socket_path.display()
                );
                if let Err(e) = connect_and_stream(&socket_path, Arc::clone(&bus)).await {
                    warn!(
                        "Conexión con Niri finalizada o interrumpida: {}. Reintentando en 3s...",
                        e
                    );
                }
            }
            None => {
                debug!("Socket de Niri no detectado todavía. Esperando...");
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}

pub fn find_niri_socket() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("NIRI_SOCKET") {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }

    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let dir = PathBuf::from(runtime_dir);
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Some(file_name) = p.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with("niri") && file_name.ends_with(".sock") {
                        return Some(p);
                    }
                }
            }
        }
    }

    None
}

async fn connect_and_stream(socket_path: &PathBuf, bus: Arc<EventBus>) -> Result<()> {
    let mut stream = UnixStream::connect(socket_path)
        .await
        .with_context(|| format!("No se pudo conectar a {}", socket_path.display()))?;

    // Handshake Niri IPC: solicitamos el stream de eventos enviando "\"EventStream\"\n"
    stream.write_all(b"\"EventStream\"\n").await?;
    stream.flush().await?;

    let (reader, _) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    // La primera línea es la respuesta de confirmación {"Ok":"Handled"}
    if buf_reader.read_line(&mut line).await? > 0 {
        debug!("Handshake de Niri IPC: {}", line.trim());
        line.clear();
    }

    let mut known_windows: HashMap<u64, (String, String, u32)> = HashMap::new();

    while buf_reader.read_line(&mut line).await? > 0 {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            process_niri_line(trimmed, &mut known_windows, &bus);
        }
        line.clear();
    }

    Ok(())
}

pub fn process_niri_line(
    line: &str,
    known_windows: &mut HashMap<u64, (String, String, u32)>,
    bus: &EventBus,
) {
    let parsed: Result<NiriRawEvent, _> = serde_json::from_str(line);
    match parsed {
        Ok(NiriRawEvent::WindowOpenedOrChanged { data }) => {
            let win = data.window;
            let app_id = win.app_id.unwrap_or_else(|| "unknown".to_string());
            let title = win.title.unwrap_or_default();
            let workspace = win.workspace_id.unwrap_or(1);

            known_windows.insert(win.id, (app_id.clone(), title.clone(), workspace));

            if win.is_focused {
                bus.publish(SystemEvent::WindowFocusChanged {
                    app_id,
                    title,
                    workspace,
                });
            }
        }
        Ok(NiriRawEvent::WindowFocusChanged { data }) => {
            if let Some(focused_id) = data.id {
                if let Some((app_id, title, workspace)) = known_windows.get(&focused_id) {
                    bus.publish(SystemEvent::WindowFocusChanged {
                        app_id: app_id.clone(),
                        title: title.clone(),
                        workspace: *workspace,
                    });
                }
            }
        }
        Ok(NiriRawEvent::WorkspaceActivated { data }) => {
            if data.focused {
                bus.publish(SystemEvent::WorkspaceActivated {
                    workspace: data.id,
                });
            }
        }
        Ok(NiriRawEvent::Other(_)) => {
            // Ignorar otros eventos no críticos de Niri
        }
        Err(e) => {
            debug!("Línea de Niri no parseada: {} ({})", line, e);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_niri_window_opened() {
        let bus = EventBus::new(16);
        let mut rx = bus.subscribe();
        let mut known = HashMap::new();

        let json = r#"{"WindowOpenedOrChanged":{"window":{"id":42,"title":"editing file.rs","app_id":"foot","workspace_id":2,"is_focused":true}}}"#;
        process_niri_line(json, &mut known, &bus);

        let event = rx.try_recv().expect("Debe haber emitido un evento");
        if let SystemEvent::WindowFocusChanged {
            app_id,
            title,
            workspace,
        } = &*event
        {
            assert_eq!(app_id, "foot");
            assert_eq!(title, "editing file.rs");
            assert_eq!(*workspace, 2);
        } else {
            panic!("Evento inesperado: {:?}", event);
        }
    }

    #[test]
    fn test_parse_niri_workspace_activated() {
        let bus = EventBus::new(16);
        let mut rx = bus.subscribe();
        let mut known = HashMap::new();

        let json = r#"{"WorkspaceActivated":{"id":3,"focused":true}}"#;
        process_niri_line(json, &mut known, &bus);

        let event = rx.try_recv().expect("Debe haber emitido un evento");
        if let SystemEvent::WorkspaceActivated { workspace } = &*event {
            assert_eq!(*workspace, 3);
        } else {
            panic!("Evento inesperado: {:?}", event);
        }
    }
}
