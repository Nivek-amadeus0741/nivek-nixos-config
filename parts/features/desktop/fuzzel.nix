{ ... }: {
  flake.modules.homeManager.fuzzel = { pkgs, ... }: {
    programs.fuzzel = {
      enable = true;
      settings = {
        main = {
          font = "JetBrainsMono Nerd Font:size=13";
          terminal = "${pkgs.kitty}/bin/kitty -e";
          prompt = "'󰍉  '";
          layer = "overlay";
          lines = 10;
          width = 46;
          tabs = 4;
          horizontal-pad = 26;
          vertical-pad = 14;
          inner-pad = 12;
          match-counter = true;
          dpi-aware = "no";
          icons-enabled = true;
          image-size-ratio = 0.5;
        };

        border = {
          radius = 16;
          width = 2;
        };

        # Paleta Material You / Caelestia
        colors = {
          background = "101416ee";
          text = "dee3e5ff";
          prompt = "82d3e1ff";
          placeholder = "738a92ff";
          input = "dee3e5ff";
          match = "82d3e1ff";
          selection = "1f2b30ff";
          selection-text = "82d3e1ff";
          selection-match = "ffffffff";
          counter = "738a92ff";
          border = "82d3e177";
        };
      };
    };
  };
}
