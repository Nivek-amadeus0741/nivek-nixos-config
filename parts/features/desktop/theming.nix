{ ... }: {
  flake.modules.nixos.theming = { pkgs, ... }: {
    stylix = {
      enable = true;
      autoEnable = true;
      image = ../../../assets/wallpapers/caelestia-default.png;
      polarity = "dark";

      cursor = {
        package = pkgs.bibata-cursors;
        name = "Bibata-Modern-Ice";
        size = 24;
      };

      fonts = {
        monospace = {
          package = pkgs.nerd-fonts.jetbrains-mono;
          name = "JetBrainsMono Nerd Font";
        };
        sansSerif = {
          package = pkgs.noto-fonts;
          name = "Noto Sans";
        };
        serif = {
          package = pkgs.noto-fonts;
          name = "Noto Serif";
        };
        emoji = {
          package = pkgs.noto-fonts-color-emoji;
          name = "Noto Color Emoji";
        };
        sizes = {
          applications = 10;
          desktop = 10;
          terminal = 11;
          popups = 10;
        };
      };

      icons = {
        enable = true;
        package = pkgs.tela-circle-icon-theme;
        dark = "Tela-circle-dark";
        light = "Tela-circle-light";
      };

      opacity = {
        terminal = 0.88;
        popups = 0.90;
        desktop = 0.85;
      };
    };
  };

  flake.modules.homeManager.theming = { pkgs, ... }: {
    home.packages = [
      pkgs.bibata-cursors
      pkgs.tela-circle-icon-theme
      pkgs.nerd-fonts.jetbrains-mono
      pkgs.swaybg
    ];

    # Desactivamos generación automática en barras y launchers para aplicar CSS artesanal Material You
    stylix.targets = {
      waybar.enable = false;
      fuzzel.enable = false;
      mako.enable = false;
      kitty.enable = true;
    };
  };
}
