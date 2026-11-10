use serde::{Deserialize, Serialize};

/// Eventos del sistema operativo que el bus de Rust puede despachar a los suscriptores y plugins
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum SystemEvent {
    /// Cambio de ventana activa en el compositor Wayland
    WindowFocusChanged {
        app_id: String,
        title: String,
        workspace: u32,
    },
    /// Cambio de espacio de trabajo (workspace)
    WorkspaceActivated {
        workspace: u32,
    },
    /// Evento de estado de batería o energía
    PowerStateChanged {
        on_battery: bool,
        percentage: u8,
    },
    /// Solicitud de cambio de contexto operativo (ej: dev, media, gaming, studio)
    ContextRequested {
        context: String,
    },
    /// Evento genérico o de prueba
    Custom {
        topic: String,
        message: String,
    },
}

/// Acciones que un cliente o plugin puede ordenar al motor de sistemas
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", content = "params")]
pub enum SystemAction {
    /// Ajustar brillo de pantalla por DDC/CI (I2C)
    SetDisplayBrightness {
        display: u32,
        brightness: u8,
    },
    /// Ajustar el gobernador de la CPU (performance, powersave, schedutil, etc.)
    SetCpuGovernor {
        governor: String,
    },
    /// Enviar notificación de escritorio
    Notify {
        summary: String,
        body: String,
        urgency: String,
    },
    /// Conmutar modo de audio en PipeWire
    SetAudioProfile {
        profile: String,
    },
}

/// Solicitudes IPC enviadas por nivekctl u otros clientes hacia nivek-core
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", content = "data")]
pub enum IpcRequest {
    /// Comprobación de salud básica y medición de RTT
    Ping,
    /// Consulta del estado global del demonio y métricas de runtime
    GetStatus,
    /// Transforma la conexión en un stream continuo de eventos en tiempo real
    SubscribeEvents,
    /// Inyecta un evento directamente en el bus de eventos del sistema
    EmitEvent(SystemEvent),
    /// Ejecuta una acción sobre el sistema operativo
    TriggerAction(SystemAction),
}

/// Estado y métricas de salud del demonio nivek-core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub version: String,
    pub uptime_secs: u64,
    pub total_events_dispatched: u64,
    pub active_subscribers: usize,
    pub memory_rss_bytes: u64,
}

/// Respuestas IPC devueltas por nivek-core a los clientes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "res", content = "data")]
pub enum IpcResponse {
    Pong {
        version: String,
        uptime_secs: u64,
    },
    Status(DaemonStatus),
    EventNotification(SystemEvent),
    Ack {
        success: bool,
        message: String,
    },
    Error {
        message: String,
    },
}
