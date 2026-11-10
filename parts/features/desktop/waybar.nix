{ ... }: {
  flake.modules.homeManager.waybar = { pkgs, ... }:
    let
      nivekWaybarStatus = pkgs.writeShellApplication {
        name = "nivek-waybar-status";
        runtimeInputs = [ pkgs.coreutils pkgs.gnugrep pkgs.gawk ];
        text = ''
          if ! RAW=$(nivekctl status 2>/dev/null); then
            echo '{"text":"󰒲 Engine","tooltip":"nivek-engine no está en ejecución","class":"offline"}'
            exit 0
          fi

          RAM=$(echo "$RAW" | grep "Memoria RSS" | awk '{print $4, $5}' | tr -d '│')
          UPTIME=$(echo "$RAW" | grep "Tiempo Activo" | awk -F': ' '{print $2}' | tr -d '│ ')
          EVENTS=$(echo "$RAW" | grep "Eventos Procesados" | awk '{print $4}' | tr -d '│')

          echo "{\"text\":\"󱐋 Engine [OK]\",\"tooltip\":\"󱐋 Nivek Reactive OS Engine\\n󰍛 Memoria: $RAM\\n⏱ Uptime: $UPTIME\\n⚡ Eventos: $EVENTS\\n(Clic para abrir Monitor en vivo)\",\"class\":\"active\"}"
        '';
      };
    in {
      home.packages = with pkgs; [
        btop
        blueman
        networkmanagerapplet
        pavucontrol
      ];

      programs.waybar = {
        enable = true;
        systemd.enable = true;

        settings = {
          mainBar = {
            layer = "top";
            position = "top";
            height = 40;
            margin-top = 8;
            margin-left = 12;
            margin-right = 12;
            spacing = 6;

            modules-left = [
              "custom/launcher"
              "niri/workspaces"
              "custom/dock-kitty"
              "custom/dock-zen"
              "custom/dock-code"
              "custom/dock-obsidian"
              "custom/dock-yazi"
              "niri/window"
            ];

            modules-center = [
              "clock"
            ];

            modules-right = [
              "custom/nivek"
              "cpu"
              "memory"
              "battery"
              "backlight"
              "pulseaudio"
              "bluetooth"
              "network"
              "tray"
            ];

            "custom/launcher" = {
              format = "󰣇";
              tooltip = false;
              on-click = "${pkgs.fuzzel}/bin/fuzzel";
            };

            "niri/workspaces" = {
              format = "{index}";
              on-click = "activate";
            };

            "custom/dock-kitty" = {
              format = "󰄛";
              tooltip-format = "Terminal Kitty";
              on-click = "${pkgs.kitty}/bin/kitty";
            };

            "custom/dock-zen" = {
              format = "󰖟";
              tooltip-format = "Zen Browser";
              on-click = "zen";
            };

            "custom/dock-code" = {
              format = "󰨞";
              tooltip-format = "VS Code";
              on-click = "code";
            };

            "custom/dock-obsidian" = {
              format = "󱓧";
              tooltip-format = "Obsidian (Bóveda de Notas)";
              on-click = "obsidian";
            };

            "custom/dock-yazi" = {
              format = "󰝰";
              tooltip-format = "Yazi (Explorador de Archivos)";
              on-click = "${pkgs.kitty}/bin/kitty --class yazi -e yazi";
            };

            "niri/window" = {
              format = "󰖲  {title}";
              max-length = 26;
              separate-outputs = true;
            };

            clock = {
              interval = 1;
              format = "  {:%H:%M:%S  ·    %a %d %b}";
              tooltip-format = "<tt><small>{calendar}</small></tt>";
            };

            "custom/nivek" = {
              exec = "${nivekWaybarStatus}/bin/nivek-waybar-status";
              return-type = "json";
              interval = 3;
              on-click = "${pkgs.kitty}/bin/kitty --title 'Nivek Engine Monitor' -e nivekctl monitor";
            };

            cpu = {
              interval = 2;
              format = "󰍛  {usage}%";
              tooltip = true;
              on-click = "${pkgs.kitty}/bin/kitty -e btop";
            };

            memory = {
              interval = 2;
              format = "󰘚  {percentage}%";
              tooltip-format = "RAM: {used:0.1f}GiB / {total:0.1f}GiB ({percentage}%)\nSwap: {swapUsed:0.1f}GiB / {swapTotal:0.1f}GiB";
              on-click = "${pkgs.kitty}/bin/kitty -e btop";
            };

            battery = {
              states = {
                warning = 30;
                critical = 15;
              };
              format = "{icon}  {capacity}%";
              format-charging = "󰂄  {capacity}%";
              format-plugged = "󰚥  {capacity}%";
              format-icons = [ "󰂎" "󰁺" "󰁻" "󰁼" "󰁽" "󰁾" "󰁿" "󰂀" "󰂁" "󰂂" "󰁹" ];
              tooltip-format = "{timeTo}, {capacity}%";
            };

            backlight = {
              format = "{icon}  {percent}%";
              format-icons = [ "󰃞" "󰃟" "󰃠" ];
              on-scroll-up = "nivekctl action set-brightness 5";
              on-scroll-down = "nivekctl action set-brightness -5";
            };

            pulseaudio = {
              format = "{icon}  {volume}%";
              format-muted = "󰝟  Mute";
              format-icons = {
                default = [ "󰕿" "󰖀" "󰕾" ];
              };
              scroll-step = 5;
              on-click = "${pkgs.pavucontrol}/bin/pavucontrol";
            };

            bluetooth = {
              format = "󰂯 {status}";
              format-connected = "󰂱 {device_alias}";
              format-connected-battery = "󰂱 {device_alias} ({device_battery_percentage}%)";
              tooltip-format = "{controller_alias}\t{controller_address}\n\n{num_connections} conectados";
              tooltip-format-connected = "{controller_alias}\t{controller_address}\n\n{num_connections} conectados\n\n{device_enumerate}";
              tooltip-format-enumerate-connected = "{device_alias}\t{device_address}";
              on-click = "${pkgs.blueman}/bin/blueman-manager";
            };

            network = {
              format-wifi = "󰤨  {essid}";
              format-ethernet = "󰈀  {ipaddr}";
              format-linked = "󰈀  (Sin IP)";
              format-disconnected = "󰤮  Desconectado";
              tooltip-format-wifi = "Red: {essid} ({signalStrength}%)\nIP: {ipaddr}\nTráfico: ↑{bandwidthUpBits} | ↓{bandwidthDownBits}";
              tooltip-format-ethernet = "Interfaz: {ifname}\nIP: {ipaddr}\nTráfico: ↑{bandwidthUpBits} | ↓{bandwidthDownBits}";
              on-click = "${pkgs.networkmanagerapplet}/bin/nm-connection-editor";
            };

            tray = {
              icon-size = 15;
              spacing = 8;
            };
          };
        };

        style = ''
          * {
            font-family: "JetBrainsMono Nerd Font", "Noto Sans", sans-serif;
            font-size: 13px;
            font-weight: 500;
            border: none;
            border-radius: 0;
            min-height: 0;
          }

          window#waybar {
            background-color: transparent;
            color: #dee3e5;
          }

          /* ========================================================
             ESTÉTICA GLASSMORPHISM (Islas translúcidas con blur)
             ======================================================== */
          #custom-launcher,
          #workspaces,
          #custom-dock-kitty,
          #custom-dock-zen,
          #custom-dock-code,
          #custom-dock-obsidian,
          #custom-dock-yazi,
          #window,
          #clock,
          #custom-nivek,
          #cpu,
          #memory,
          #battery,
          #backlight,
          #pulseaudio,
          #bluetooth,
          #network,
          #tray {
            background: rgba(16, 22, 28, 0.65);
            backdrop-filter: blur(24px) saturate(180%);
            -webkit-backdrop-filter: blur(24px) saturate(180%);
            border: 1px solid rgba(255, 255, 255, 0.12);
            box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
            border-radius: 16px;
            padding: 4px 12px;
            margin: 2px 2px;
            transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
          }

          /* Hover general con brillo sutil */
          #custom-launcher:hover,
          #workspaces:hover,
          #clock:hover,
          #custom-nivek:hover,
          #cpu:hover,
          #memory:hover,
          #battery:hover,
          #backlight:hover,
          #pulseaudio:hover,
          #bluetooth:hover,
          #network:hover {
            background: rgba(24, 32, 40, 0.80);
            border-color: rgba(130, 211, 225, 0.40);
            box-shadow: 0 8px 24px 0 rgba(130, 211, 225, 0.15);
          }

          /* Lanzador del Menú */
          #custom-launcher {
            color: #82d3e1;
            font-size: 16px;
            padding: 4px 10px;
          }

          /* Workspaces */
          #workspaces button {
            padding: 0 8px;
            color: #8a9ba2;
            border-radius: 10px;
            transition: all 0.2s ease-in-out;
          }

          #workspaces button.active {
            color: #101416;
            background: linear-gradient(135deg, #82d3e1, #bbc6ea);
            font-weight: 700;
            box-shadow: 0 2px 8px rgba(130, 211, 225, 0.35);
          }

          #workspaces button:hover {
            color: #dee3e5;
            background-color: rgba(130, 211, 225, 0.18);
          }

          /* Dock Launchers */
          #custom-dock-kitty,
          #custom-dock-zen,
          #custom-dock-code,
          #custom-dock-obsidian,
          #custom-dock-yazi {
            font-size: 15px;
            padding: 4px 8px;
            color: #dee3e5;
          }

          #custom-dock-kitty:hover { color: #f0bd8d; }
          #custom-dock-zen:hover { color: #82d3e1; }
          #custom-dock-code:hover { color: #bbc6ea; }
          #custom-dock-obsidian:hover { color: #d4bbff; }
          #custom-dock-yazi:hover { color: #a0d8b3; }

          /* Título de Ventana */
          #window {
            color: #b1cbd0;
          }

          /* Reloj Central */
          #clock {
            color: #82d3e1;
            font-weight: 600;
            border-color: rgba(130, 211, 225, 0.30);
            padding: 4px 16px;
          }

          /* Módulo Nivek Reactive OS Engine */
          #custom-nivek {
            color: #a0d8b3;
            border-color: rgba(160, 216, 179, 0.30);
            font-weight: 600;
          }

          #custom-nivek.offline {
            color: #ffb4ab;
            border-color: rgba(255, 180, 171, 0.35);
          }

          /* Widgets de Telemetría de Sistema */
          #cpu {
            color: #f4b5c7;
          }

          #memory {
            color: #bbc6ea;
          }

          #battery {
            color: #82d3e1;
          }

          #battery.warning {
            color: #f0bd8d;
            border-color: rgba(240, 189, 141, 0.5);
          }

          #battery.critical {
            color: #ffb4ab;
            border-color: rgba(255, 180, 171, 0.6);
            animation: blink 1s linear infinite;
          }

          @keyframes blink {
            to {
              background-color: rgba(255, 180, 171, 0.3);
            }
          }

          /* Hardware y Controles */
          #backlight {
            color: #f0bd8d;
          }

          #pulseaudio {
            color: #bbc6ea;
          }

          #bluetooth {
            color: #82d3e1;
          }

          #network {
            color: #a0d8b3;
          }

          #tray {
            padding: 4px 10px;
          }
        '';
      };
    };
}
