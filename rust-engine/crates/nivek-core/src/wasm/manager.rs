use crate::bus::EventBus;
use crate::wasm::plugin::WasmPlugin;
use nivek_wasm_abi::SystemEvent;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::broadcast;
use tracing::{error, info};

pub struct PluginManager {
    rules_dir: PathBuf,
    bus: Arc<EventBus>,
    plugins: HashMap<PathBuf, (SystemTime, WasmPlugin)>,
}

impl PluginManager {
    pub fn new(rules_dir: PathBuf, bus: Arc<EventBus>) -> Self {
        Self {
            rules_dir,
            bus,
            plugins: HashMap::new(),
        }
    }

    pub fn scan_and_reload(&mut self) {
        if !self.rules_dir.exists() {
            let _ = std::fs::create_dir_all(&self.rules_dir);
            return;
        }

        let entries = match std::fs::read_dir(&self.rules_dir) {
            Ok(e) => e,
            Err(err) => {
                error!("Error leyendo directorio de plugins WASM: {}", err);
                return;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext == "wat" || ext == "wasm" {
                    let mtime = entry
                        .metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(SystemTime::UNIX_EPOCH);

                    let needs_load = match self.plugins.get(&path) {
                        Some((last_mtime, _)) => mtime > *last_mtime,
                        None => true,
                    };

                    if needs_load {
                        match WasmPlugin::load(path.clone(), Arc::clone(&self.bus)) {
                            Ok(plugin) => {
                                info!(
                                    "🔌 [WASM MANAGER] Plugin cargado/recargado en caliente: '{}'",
                                    plugin.name
                                );
                                self.plugins.insert(path, (mtime, plugin));
                            }
                            Err(e) => {
                                error!("Error cargando plugin WASM {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn dispatch_event(&mut self, event: &SystemEvent) {
        match event {
            SystemEvent::WindowFocusChanged { workspace, .. } => {
                for (_mtime, plugin) in self.plugins.values_mut() {
                    plugin.on_window_focus(*workspace as i32);
                }
            }
            SystemEvent::WorkspaceActivated { workspace } => {
                for (_mtime, plugin) in self.plugins.values_mut() {
                    plugin.on_window_focus(*workspace as i32);
                }
            }
            SystemEvent::PowerStateChanged { percentage, .. } => {
                for (_mtime, plugin) in self.plugins.values_mut() {
                    plugin.on_battery(*percentage as i32);
                }
            }
            _ => {}
        }
    }
}

pub async fn run_wasm_manager(
    rules_dir: PathBuf,
    bus: Arc<EventBus>,
    mut receiver: broadcast::Receiver<Arc<SystemEvent>>,
) {
    info!(
        "Iniciando gestor de plugins WASM (directorio: {})...",
        rules_dir.display()
    );

    let mut manager = PluginManager::new(rules_dir, Arc::clone(&bus));
    manager.scan_and_reload();

    let mut scan_interval = tokio::time::interval(Duration::from_secs(3));

    loop {
        tokio::select! {
            _ = scan_interval.tick() => {
                manager.scan_and_reload();
            }
            event_res = receiver.recv() => {
                if let Ok(event) = event_res {
                    manager.dispatch_event(&event);
                }
            }
        }
    }
}
