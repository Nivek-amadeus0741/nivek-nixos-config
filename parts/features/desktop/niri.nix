{ ... }: {
  flake.modules.nixos.niri = { pkgs, ... }: {
    programs.niri = {
      enable = true;
      package = pkgs.niri;
    };

    security.polkit.enable = true;
    services.gnome.gnome-keyring.enable = true;

    xdg.portal = {
      enable = true;
      extraPortals = [
        pkgs.xdg-desktop-portal-gnome
      ];
    };
  };

  flake.modules.homeManager.niri = { pkgs, ... }: {
    home.packages = with pkgs; [
      brightnessctl
      grim
      slurp
      wl-clipboard
      playerctl
      pavucontrol
      polkit_gnome
      swaybg
      xwayland-satellite
    ];

    home.file.".local/share/wallpapers/caelestia-default.png".source = ../../../assets/wallpapers/caelestia-default.png;

    xdg.configFile."niri/config.kdl".text = ''
      // Niri Wayland Compositor - Configuración Caelestia / Material You
      // Integrada con Nivek Reactive OS Engine (Rust)

      prefer-no-csd

      environment {
        NIXOS_OZONE_WL "1"
        MOZ_ENABLE_WAYLAND "1"
        ELECTRON_OZONE_PLATFORM_HINT "wayland"
        QT_QPA_PLATFORM "wayland"
        XDG_CURRENT_DESKTOP "niri"
      }

      spawn-at-startup "systemctl" "--user" "restart" "waybar"
      spawn-at-startup "systemctl" "--user" "restart" "mako"
      spawn-at-startup "${pkgs.polkit_gnome}/libexec/polkit-gnome-authentication-agent-1"
      spawn-at-startup "${pkgs.swaybg}/bin/swaybg" "-i" "/home/nivek/.local/share/wallpapers/caelestia-default.png" "-m" "fill"

      input {
        keyboard {
          xkb {
            layout "us,latam"
            options "grp:win_space_toggle"
          }
        }
        touchpad {
          tap
          natural-scroll
          dwt
        }
        focus-follows-mouse
      }

      layout {
        gaps 12
        center-focused-column "never"

        default-column-width { proportion 0.5; }

        preset-column-widths {
          proportion 0.33333
          proportion 0.5
          proportion 0.66667
        }

        focus-ring {
          width 2
          active-gradient from="#82d3e1" to="#bbc6ea" angle=45
          inactive-color "#1c2428cc"
        }

        border {
          off
        }
      }

      window-rule {
        geometry-corner-radius 14
        clip-to-geometry true
      }

      window-rule {
        match app-id=r#"^org\.kde\.ksshaskpass$"#
        match app-id=r#"^pavucontrol$"#
        match app-id=r#"^nm-connection-editor$"#
        match app-id=r#"^zenity$"#
        match title=r#"^Open File$"#
        match title=r#"^Save File$"#
        match title=r#"^Nivek Engine Monitor$"#
        open-floating true
      }

      window-rule {
        match app-id=r#"^zen"#
        default-column-width { proportion 0.66667; }
      }

      window-rule {
        match app-id=r#"^obsidian$"#
        default-column-width { proportion 0.5; }
      }

      window-rule {
        match app-id=r#"^yazi$"#
        default-column-width { proportion 0.5; }
      }

      binds {
        // Lanzadores y Terminales
        Mod+Return { spawn "${pkgs.kitty}/bin/kitty"; }
        Mod+Shift+Return { spawn "${pkgs.wezterm}/bin/wezterm"; }
        Mod+Space { spawn "${pkgs.fuzzel}/bin/fuzzel"; }
        Mod+D { spawn "${pkgs.fuzzel}/bin/fuzzel"; }

        // Aplicaciones Principales
        Mod+B { spawn "zen"; }
        Mod+E { spawn "code"; }
        Mod+O { spawn "obsidian"; }
        Mod+N { spawn "${pkgs.kitty}/bin/kitty" "--class" "yazi" "-e" "yazi"; }

        // Gestión de Ventanas
        Mod+Q { close-window; }
        Mod+Shift+E { quit; }

        // Foco de Navegación (Flechas y Vim keys)
        Mod+Left  { focus-column-left; }
        Mod+Right { focus-column-right; }
        Mod+Down  { focus-window-down; }
        Mod+Up    { focus-window-up; }
        Mod+H     { focus-column-left; }
        Mod+L     { focus-column-right; }
        Mod+J     { focus-window-down; }
        Mod+K     { focus-window-up; }

        // Desplazamiento de Columnas y Ventanas
        Mod+Shift+Left  { move-column-left; }
        Mod+Shift+Right { move-column-right; }
        Mod+Shift+Down  { move-window-down; }
        Mod+Shift+Up    { move-window-up; }
        Mod+Shift+H     { move-column-left; }
        Mod+Shift+L     { move-column-right; }
        Mod+Shift+J     { move-window-down; }
        Mod+Shift+K     { move-window-up; }

        // Workspaces
        Mod+1 { focus-workspace 1; }
        Mod+2 { focus-workspace 2; }
        Mod+3 { focus-workspace 3; }
        Mod+4 { focus-workspace 4; }
        Mod+5 { focus-workspace 5; }
        Mod+6 { focus-workspace 6; }
        Mod+7 { focus-workspace 7; }
        Mod+8 { focus-workspace 8; }
        Mod+9 { focus-workspace 9; }

        Mod+Shift+1 { move-column-to-workspace 1; }
        Mod+Shift+2 { move-column-to-workspace 2; }
        Mod+Shift+3 { move-column-to-workspace 3; }
        Mod+Shift+4 { move-column-to-workspace 4; }
        Mod+Shift+5 { move-column-to-workspace 5; }
        Mod+Shift+6 { move-column-to-workspace 6; }
        Mod+Shift+7 { move-column-to-workspace 7; }
        Mod+Shift+8 { move-column-to-workspace 8; }
        Mod+Shift+9 { move-column-to-workspace 9; }

        // Geometría y Layout
        Mod+R { switch-preset-column-width; }
        Mod+F { maximize-column; }
        Mod+Shift+F { fullscreen-window; }
        Mod+C { center-column; }
        Mod+Minus { set-column-width "-10%"; }
        Mod+Equal { set-column-width "+10%"; }
        Mod+V { toggle-window-floating; }
        Mod+Shift+V { switch-focus-between-floating-and-tiling; }

        // Control Multimedia / Hardware
        XF86AudioRaiseVolume allow-when-locked=true { spawn "nivekctl" "action" "set-volume" "+5%"; }
        XF86AudioLowerVolume allow-when-locked=true { spawn "nivekctl" "action" "set-volume" "-5%"; }
        XF86AudioMute allow-when-locked=true        { spawn "nivekctl" "action" "toggle-mute"; }
        XF86MonBrightnessUp allow-when-locked=true  { spawn "brightnessctl" "set" "+5%"; }
        XF86MonBrightnessDown allow-when-locked=true { spawn "brightnessctl" "set" "5%-"; }

        // Capturas de Pantalla
        Print { spawn "sh" "-c" "grim -g \"$(slurp)\" - | wl-copy"; }
        Mod+Shift+S { spawn "sh" "-c" "grim -g \"$(slurp)\" - | wl-copy"; }
      }
    '';
  };
}
