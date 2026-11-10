use crate::bus::EventBus;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;
use wasmi::{Caller, Engine, Func, Linker, Module, Store};

pub struct HostContext {
    pub plugin_name: String,
    pub bus: Arc<EventBus>,
}

pub struct WasmPlugin {
    pub name: String,
    #[allow(dead_code)]
    pub path: Option<PathBuf>,
    store: Store<HostContext>,
    instance: wasmi::Instance,
}

impl WasmPlugin {
    pub fn load(path: PathBuf, bus: Arc<EventBus>) -> Result<Self> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let bytes = std::fs::read(&path)
            .with_context(|| format!("No se pudo leer plugin en {}", path.display()))?;

        let mut plugin = Self::load_from_bytes(name, &bytes, bus)?;
        plugin.path = Some(path);
        Ok(plugin)
    }

    pub fn load_from_bytes(name: String, bytes: &[u8], bus: Arc<EventBus>) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::new(&engine, bytes)
            .with_context(|| format!("No se pudo parsear módulo WASM/WAT para '{}'", name))?;

        let context = HostContext {
            plugin_name: name.clone(),
            bus,
        };

        let mut store = Store::new(&engine, context);
        let mut linker = Linker::new(&engine);

        let set_brightness_func = Func::wrap(
            &mut store,
            |caller: Caller<'_, HostContext>, display: i32, brightness: i32| {
                caller
                    .data()
                    .bus
                    .publish(nivek_wasm_abi::SystemEvent::Custom {
                        topic: "wasm.action.brightness".to_string(),
                        message: format!(
                            "Plugin '{}' ajustó brillo de pantalla {} a {}%",
                            caller.data().plugin_name,
                            display,
                            brightness
                        ),
                    });
            },
        );

        let log_msg_func = Func::wrap(
            &mut store,
            |_caller: Caller<'_, HostContext>, _ptr: i32, _len: i32| {
                info!("Mensaje de log desde plugin WASM");
            },
        );

        linker.define("env", "set_brightness", set_brightness_func)?;
        linker.define("env", "log_msg", log_msg_func)?;

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .with_context(|| format!("Fallo al instanciar plugin WASM '{}'", name))?;

        info!("Plugin WASM inicializado: '{}'", name);

        Ok(Self {
            name,
            path: None,
            store,
            instance,
        })
    }

    pub fn on_window_focus(&mut self, workspace: i32) {
        if let Some(func) = self.instance.get_func(&self.store, "on_window_focus") {
            if let Ok(typed) = func.typed::<i32, ()>(&self.store) {
                let _ = typed.call(&mut self.store, workspace);
            }
        }
    }

    pub fn on_battery(&mut self, percentage: i32) {
        if let Some(func) = self.instance.get_func(&self.store, "on_battery") {
            if let Ok(typed) = func.typed::<i32, ()>(&self.store) {
                let _ = typed.call(&mut self.store, percentage);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WAT: &[u8] = br#"(module
      (import "env" "set_brightness" (func $set_brightness (param i32 i32)))
      (import "env" "log_msg" (func $log_msg (param i32 i32)))
      (func (export "on_window_focus") (param $workspace i32)
        (if (i32.eq (local.get $workspace) (i32.const 2))
          (then
            (call $set_brightness (i32.const 1) (i32.const 40))
            (call $log_msg (i32.const 0) (i32.const 33))
          )
        )
      )
      (func (export "on_battery") (param $percentage i32)
        (if (i32.le_s (local.get $percentage) (i32.const 15))
          (then
            (call $set_brightness (i32.const 1) (i32.const 25))
          )
        )
      )
    )"#;

    #[test]
    fn test_wasm_plugin_execution() {
        let bus = Arc::new(EventBus::new(16));
        let mut rx = bus.subscribe();

        let mut plugin = WasmPlugin::load_from_bytes(
            "auto_dimmer.wat".to_string(),
            TEST_WAT,
            Arc::clone(&bus),
        )
        .expect("Debe cargar auto_dimmer.wat desde memoria");

        // Disparamos evento de foco en workspace 2 (debe atenuar al 40%)
        plugin.on_window_focus(2);

        let event = rx.try_recv().expect("Debe haber emitido evento desde WASM");
        if let nivek_wasm_abi::SystemEvent::Custom { topic, message } = &*event {
            assert_eq!(topic, "wasm.action.brightness");
            assert!(message.contains("ajustó brillo de pantalla 1 a 40%"));
        } else {
            panic!("Evento inesperado desde WASM: {:?}", event);
        }

        // Disparamos evento de batería baja al 12% (debe atenuar al 25%)
        plugin.on_battery(12);
        let event2 = rx
            .try_recv()
            .expect("Debe haber emitido segundo evento desde WASM");
        if let nivek_wasm_abi::SystemEvent::Custom { topic, message } = &*event2 {
            assert_eq!(topic, "wasm.action.brightness");
            assert!(message.contains("ajustó brillo de pantalla 1 a 25%"));
        } else {
            panic!("Evento inesperado desde WASM: {:?}", event2);
        }
    }
}
