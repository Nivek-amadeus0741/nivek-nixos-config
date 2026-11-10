{ ... }: {
  flake.modules.homeManager.terminals = { pkgs, lib, ... }: {
    # Kitty: Terminal acelerado por GPU de alto rendimiento
    programs.kitty = {
      enable = true;
      settings = {
        font_size = "11.5";
        window_padding_width = 12;
        background_opacity = lib.mkForce "0.88";
        background_blur = 30;
        confirm_os_window_close = 0;
        enable_audio_bell = false;
        cursor_shape = "beam";
        cursor_blink_interval = "0.5";
        scrollback_lines = 10000;
        copy_on_select = "clipboard";
        tab_bar_style = "powerline";
        tab_powerline_style = "slanted";
      };
    };

    # Wezterm: Multiplexor y terminal extensible con Lua
    programs.wezterm = {
      enable = true;
      extraConfig = ''
        local wezterm = require 'wezterm'
        local config = wezterm.config_builder()

        config.font = wezterm.font('JetBrainsMono Nerd Font')
        config.font_size = 11.5
        config.window_background_opacity = 0.88
        config.window_padding = {
          left = 12,
          right = 12,
          top = 12,
          bottom = 12,
        }
        config.enable_tab_bar = true
        config.use_fancy_tab_bar = false
        config.hide_tab_bar_if_only_one_tab = true
        config.window_close_confirmation = 'NeverPrompt'
        config.default_cursor_style = 'BlinkingBar'
        config.animation_fps = 60

        return config
      '';
    };
  };
}
