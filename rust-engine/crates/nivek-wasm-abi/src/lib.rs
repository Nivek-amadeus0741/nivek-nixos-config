use serde::{Deserialize, Serialize};

/// Eventos del sistema operativo que el bus de Rust puede despachar a los plugins WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Notificación de cambio de contexto solicitada por el usuario o script
    ContextRequested {
        context: String,
    },
}

/// Acciones que un plugin o regla puede ordenar al motor de sistemas
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "params")]
pub enum SystemAction {
    /// Ajustar brillo de pantalla por DDC/CI (I2C)
    SetDisplayBrightness {
        display: u32,
        brightness: u8,
    },
    /// Ajustar el gobernador de la CPU (performance, powersave, etc.)
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
