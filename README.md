# 🌌 Nivek Reactive OS — NixOS Dendritic Laboratory

<div align="center">

![NixOS](https://img.shields.io/badge/NixOS-Unstable-5277C3?style=for-the-badge&logo=nixos&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-2021%20Edition-DEA584?style=for-the-badge&logo=rust&logoColor=black)
![Tokio](https://img.shields.io/badge/Async-Tokio-000000?style=for-the-badge)
![WASM](https://img.shields.io/badge/Extensibility-wasmi%202.0-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Wayland](https://img.shields.io/badge/Wayland-Niri-329632?style=for-the-badge)
![Style](https://img.shields.io/badge/Theme-Caelestia%20%2F%20Material%20You-82D3E1?style=for-the-badge)

*Un sistema operativo declarativo, inmutable y reactivo impulsado por NixOS, con un motor de sistemas asíncrono en Rust y extensibilidad en WebAssembly.*

</div>

---

## ⚡ Filosofía Arquitectónica

Este repositorio implementa una **Arquitectura Dendrítica** modular en NixOS que desacopla la configuración declarativa del sistema operativo de la reactividad en tiempo de ejecución. 

En lugar de depender de scripts lentos o sobrecargar la evaluación de Nix con impurezas, un **motor asíncrono en Rust puro (`nivek-core`)** se ejecuta como demonio de usuario, supervisando eventos de ventanas en Wayland, controlando el enrutamiento de audio en PipeWire, monitorizando el estado de batería y hardware vía sysfs, y permitiendo la inyección de reglas dinámicas en caliente mediante **WebAssembly (`wasmi 2.0`)**.

---

## 💎 Características Principales

### 1. 🚀 Motor de Sistemas Asíncrono en Rust (`nivek-engine`)
* **Protocolo IPC Tipado**: Comunicación multicliente mediante socket UNIX (`$XDG_RUNTIME_DIR/nivek-engine.sock`) con latencia RTT de **585 microsegundos**.
* **Consumo Mínimo de Recursos**: Huella de memoria constante inferior a **10 MB RSS**.
* **Bus de Difusión Pub/Sub**: Basado en `tokio::sync::broadcast` con eventos clonables en memoria envueltos en `Arc<SystemEvent>`.
* **Colector Niri Wayland**: Suscripción directa al `EventStream` de Niri para capturar aperturas de ventanas, cambios de espacio de trabajo y pantalla completa.
* **Morfismo Contextual**: Alternancia dinámica entre perfiles del sistema (`dev`, `media`, `gaming`, `balanced`) ajustando gobernadores de CPU y perfiles de PipeWire en tiempo real.
* **Extensibilidad WASM**: Motor `wasmi 2.0` con vigilancia de directorio (`~/.config/nivek/rules/`) que compila e instancía archivos `.wat` y `.wasm` en caliente sin reiniciar el demonio.

### 2. 🎨 Experiencia Visual y Estética Glassmorphism
* **Gestor de Inicio de Sesión Gráfico**: **SDDM** en Wayland nativo con tematización automática por Stylix e inicio directo en sesión **Niri**.
* **Compositor Niri**: Mosaico con scroll infinito, esquinas redondeadas (14px), espaciado (*gaps*) de 12px y anillos de foco dinámicos con degradado Cyan/Lavanda (`#82d3e1` $\rightarrow$ `#bbc6ea`).
* **Panel Superior & Dock en Waybar**:
  * **Estilo Glassmorphism**: Fondo translúcido con desenfoque de fondo profundo (`backdrop-filter: blur(24px)`), bordes esmerilados y sombras suaves.
  * **Dock Integrado**: Iconos de acceso rápido para Kitty, Zen Browser, VS Code, Obsidian y Yazi con efectos hover.
  * **Widgets de Control**: Monitor de telemetría de red WiFi/Ethernet, Bluetooth con gestión de dispositivos conectados, reloj central, nivel y tiempo de batería, porcentaje de memoria RAM, uso de CPU y el módulo reactivo **󱐋 Nivek Engine** (con telemetría y apertura interactiva de monitor).
* **Tematización Unificada con Stylix**: Paleta generada a partir del fondo de pantalla oficial Caelestia 4K, con fuentes `JetBrainsMono Nerd Font` e iconos `Tela-circle-dark`.

### 3. 🛠️ Suite de Aplicaciones y Terminales
* **Shell Interactivo**: **Zsh** como intérprete predeterminado en ambas terminales, potenciado con **Starship Prompt**, completado inteligente y resaltado de sintaxis.
* **Terminales**: **Kitty** (acelerado por GPU) y **Wezterm** (multiplexor en Lua), ambos con opacidad del 88% y desenfoque.
* **Navegación Web**: **Zen Browser** oficial (`zen-browser-flake`) con optimizaciones nativas para Wayland.
* **Gestor de Archivos**: **Yazi** preconfigurado como explorador de archivos predeterminado del sistema (`inode/directory`).
* **Desarrollo**: **VS Code** (con perfiles de Home Manager y extensiones para Nix y Rust) y **Neovim** (configurado en Lua con Treesitter, Telescope, Lualine, Which-Key y Gitsigns).
* **Gestión de Conocimiento**: **Obsidian** integrado con reglas de pantalla dedicadas.
* **Asistencia Agéntica**: **Antigravity CLI (`agy`)** compatible out-of-the-box mediante `nix-ld` y wrappers en `$PATH`.
* **Herramientas Git**: **Delta** con resaltado sintáctico, TUI **Lazygit** con tema Cyan y **GitHub CLI (`gh`)**.

---

## ⌨️ Atajos de Teclado (Cheat Sheet)

> `Mod` = Tecla **Super / Windows**.

| Atajo | Acción | Destino / Detalle |
| :--- | :--- | :--- |
| `Mod + Return` | Abrir Terminal | **Kitty** (GPU, Zsh, Starship) |
| `Mod + Shift + Return` | Terminal Alternativo | **Wezterm** |
| `Mod + Space` / `Mod + D` | Lanzador de Apps | **Fuzzel** (Material You) |
| `Mod + B` | Navegador Web | **Zen Browser** (Wayland) |
| `Mod + E` | Entorno IDE | **VS Code** |
| `Mod + O` | Bóveda de Notas | **Obsidian** |
| `Mod + N` | Explorador de Archivos | **Yazi** en Kitty |
| `Mod + Q` | Cerrar Ventana | Ventana activa |
| `Mod + H / J / K / L` | Navegación | Izquierda / Abajo / Arriba / Derecha |
| `Mod + Shift + H / J / K / L` | Mover Columnas | Desplazar ventana o columna |
| `Mod + 1` .. `9` | Espacios de Trabajo | Cambiar al Workspace 1 .. 9 |
| `Mod + Shift + 1` .. `9` | Mover a Workspace | Mover columna activa al Workspace 1 .. 9 |
| `Mod + R` | Ancho de Columna | Alternar 33% / 50% / 66% |
| `Mod + F` / `Shift + F` | Maximizar / Fullscreen | Columna / Pantalla completa |
| `Mod + V` | Alternar Flotante | Conmutar modo flotante en ventana activa |
| `Print` / `Mod + Shift + S` | Captura de Pantalla | Selección con `slurp` + `grim` a portapapeles |
| `XF86Audio*` | Control de Audio | Modificar volumen o silenciar vía `nivekctl` |
| `XF86MonBrightness*` | Control de Brillo | Ajustar retroiluminación con `brightnessctl` |

---

## 📁 Estructura del Repositorio

```text
nixos-config/
├── assets/
│   └── wallpapers/            # Fondos de pantalla 4K (Caelestia oficial)
├── docs/
│   └── architecture.md        # Documentación técnica y diagramas
├── parts/
│   ├── _hosts/
│   │   └── default/           # Host 'nixos-lab' (hardware-configuration.nix)
│   ├── features/
│   │   ├── apps/              # Zen Browser, Obsidian, Yazi, Zsh, Antigravity
│   │   ├── core/              # Boot, Nix settings (nix-ld), Users
│   │   ├── desktop/           # Niri KDL, Waybar Glassmorphism, Fuzzel, Mako, Stylix, SDDM
│   │   ├── dev/               # Editors (Neovim, VS Code), Git (Delta, Lazygit)
│   │   ├── hardware/          # PipeWire, Power/Brightness, Bluetooth
│   │   └── services/          # Servicio systemd de nivek-engine
│   ├── hosts.nix              # Definición automática de hosts
│   ├── modules.nix            # Agregador de módulos dendríticos
│   └── rust.nix               # Integración hermética de Crane para compilar Rust
├── rust-engine/               # Cargo Workspace en Rust puro
│   ├── crates/
│   │   ├── nivek-wasm-abi/    # Tipos formales de IPC y ABI de plugins
│   │   ├── nivek-core/        # Demonio Tokio (Bus Pub/Sub, SysFS, PipeWire, WASM)
│   │   └── nivek-cli/         # CLI interactiva (nivekctl)
│   └── plugins/               # Plugins de ejemplo en WebAssembly (.wat/.wasm)
├── flake.nix                  # Flake raíz (flake-parts + crane + stylix + zen)
└── flake.lock                 # Versiones fijadas y reproducibles
```

---

## 🚀 Despliegue e Instalación

### 1. Clonar en la máquina física con NixOS
```bash
git clone git@github.com:Nivek-amadeus0741/nivek-nixos-config.git ~/nixos-config
cd ~/nixos-config
```

### 2. Generar el hardware del host real
```bash
nixos-generate-config --show-hardware-config > parts/_hosts/default/hardware-configuration.nix
```

### 3. Compilar y Activar el Sistema
```bash
# Con la herramienta nh (recomendado):
nh os switch .

# O con nixos-rebuild nativo:
sudo nixos-rebuild switch --flake .#nixos-lab
```

### 4. Validar el Motor Reactivo
```bash
# Comprobar estado del servicio
systemctl --user status nivek-engine

# Estado del motor e IPC
nivekctl status

# Monitor de eventos en vivo
nivekctl monitor
```

---

## 🧩 Escribir Plugins en WebAssembly

Cualquier archivo `.wat` o `.wasm` colocado en `~/.config/nivek/rules/` se carga en caliente automáticamente:

```wat
;; auto_dimmer.wat
(module
  (import "env" "set_brightness" (func $set_brightness (param i32)))
  (import "env" "log_msg" (func $log_msg (param i32 i32)))

  (data (i32.const 0) "Regla WASM: Ajustando brillo por batería baja")

  (func (export "on_battery") (param $capacity i32) (param $is_charging i32)
    (if (and (i32.lt_s (local.get $capacity) (i32.const 20))
             (i32.eq (local.get $is_charging) (i32.const 0)))
      (then
        (call $log_msg (i32.const 0) (i32.const 45))
        (call $set_brightness (i32.const -15))
      )
    )
  )
)
```

---

<div align="center">
Desarrollado y mantenido con rigor de ingeniería por <b>Albert Agy</b> & <b>Nivek</b>.
</div>
