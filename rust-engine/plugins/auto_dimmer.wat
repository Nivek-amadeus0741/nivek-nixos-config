(module
  ;; Funciones del anfitrión importadas desde nivek-core
  (import "env" "set_brightness" (func $set_brightness (param i32 i32)))
  (import "env" "log_msg" (func $log_msg (param i32 i32)))

  ;; Memoria exportada
  (memory (export "memory") 1)
  (data (i32.const 0) "Auto-Dimmer WASM Plugin Triggered")

  ;; Hook invocado al cambiar el espacio de trabajo o ventana
  (func (export "on_window_focus") (param $workspace i32)
    ;; Si el espacio de trabajo es 2 (p. ej. Cine / Multimedia), atenúa el monitor 1 al 40%
    (if (i32.eq (local.get $workspace) (i32.const 2))
      (then
        (call $set_brightness (i32.const 1) (i32.const 40))
        (call $log_msg (i32.const 0) (i32.const 33))
      )
    )
  )

  ;; Hook invocado ante eventos de energía y batería
  (func (export "on_battery") (param $percentage i32)
    ;; Si la batería cae por debajo del 15%, atenúa automáticamente al 25% para ahorrar energía
    (if (i32.le_s (local.get $percentage) (i32.const 15))
      (then
        (call $set_brightness (i32.const 1) (i32.const 25))
      )
    )
  )
)
