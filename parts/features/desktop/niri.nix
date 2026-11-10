{ ... }: {
  flake.modules.nixos.niri = { pkgs, ... }: {
    programs.niri = {
      enable = true;
      package = pkgs.niri;
    };

    # Soporte esencial para sesión Wayland moderna
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
    home.packages = [
      pkgs.alacritty
      pkgs.fuzzel
      pkgs.waybar
      pkgs.wl-clipboard
    ];
  };
}
