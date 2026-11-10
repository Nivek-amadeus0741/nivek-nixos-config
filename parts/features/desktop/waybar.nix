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
      programs.waybar = {
        enable = true;
        systemd.enable = true;

        settings = {
          mainBar = {
            layer = "top";
            position = "top";
            height = 36;
            margin-top = 8;
            margin-left = 12;
            margin-right = 12;
            spacing = 8;

            modules-left = [
              "niri/workspaces"
              "niri/window"
            ];

            modules-center = [
              "clock"
            ];

            modules-right = [
              "custom/nivek"
              "pulseaudio"
              "backlight"
              "battery"
              "tray"
            ];

            "niri/workspaces" = {
              format = "{index}";
              on-click = "activate";
            };

            "niri/window" = {
              format = "󰖲  {title}";
              max-length = 40;
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

            pulseaudio = {
              format = "{icon}  {volume}%";
              format-muted = "󰝟  Mute";
              format-icons = {
                default = [ "󰕿" "󰖀" "󰕾" ];
              };
              scroll-step = 5;
              on-click = "${pkgs.pavucontrol}/bin/pavucontrol";
            };

            backlight = {
              device = "intel_backlight";
              format = "{icon}  {percent}%";
              format-icons = [ "󰃞" "󰃟" "󰃠" ];
              on-scroll-up = "nivekctl action set-brightness 5";
              on-scroll-down = "nivekctl action set-brightness -5";
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
            };

            tray = {
              icon-size = 16;
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

          /* Estética de Islas Flotantes (Pills) Material You / Caelestia */
          #workspaces,
          #window,
          #clock,
          #custom-nivek,
          #pulseaudio,
          #backlight,
          #battery,
          #tray {
            background-color: rgba(16, 20, 24, 0.82);
            border: 1px solid rgba(130, 211, 225, 0.22);
            border-radius: 14px;
            padding: 4px 14px;
            margin: 2px 3px;
          }

          /* Workspaces */
          #workspaces button {
            padding: 0 8px;
            color: #8a9ba2;
            border-radius: 8px;
            transition: all 0.2s ease-in-out;
          }

          #workspaces button.active {
            color: #101416;
            background: linear-gradient(135deg, #82d3e1, #bbc6ea);
            font-weight: 700;
          }

          #workspaces button:hover {
            color: #dee3e5;
            background-color: rgba(130, 211, 225, 0.15);
          }

          /* Título de Ventana */
          #window {
            color: #b1cbd0;
          }

          /* Reloj Central */
          #clock {
            color: #82d3e1;
            font-weight: 600;
            border-color: rgba(130, 211, 225, 0.35);
          }

          /* Módulo Especial Nivek Reactive OS Engine */
          #custom-nivek {
            color: #a0d8b3;
            border-color: rgba(160, 216, 179, 0.35);
            font-weight: 600;
          }

          #custom-nivek.offline {
            color: #ffb4ab;
            border-color: rgba(255, 180, 171, 0.35);
          }

          #custom-nivek:hover {
            background-color: rgba(160, 216, 179, 0.2);
          }

          /* Audio y Hardware */
          #pulseaudio {
            color: #bbc6ea;
          }

          #backlight {
            color: #f0bd8d;
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

          #tray {
            padding: 4px 10px;
          }
        '';
      };
    };
}
