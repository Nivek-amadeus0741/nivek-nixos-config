{ ... }: {
  flake.modules.nixos.networking = { pkgs, ... }: {
    networking.networkmanager = {
      enable = true;
    };

    # Herramientas esenciales para gestión de redes y Wi-Fi en entorno gráfico y terminal
    environment.systemPackages = with pkgs; [
      networkmanagerapplet # nm-connection-editor y nm-applet
      wirelesstools
      iw
    ];
  };

  flake.modules.homeManager.networking = { pkgs, ... }: {
    home.packages = with pkgs; [
      networkmanagerapplet
    ];
  };
}
