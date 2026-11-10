{ ... }: {
  flake.modules.nixos.power = { pkgs, ... }: {
    # Perfiles de energía y herramientas de control de brillo / hardware
    services.power-profiles-daemon.enable = true;
    services.upower.enable = true;

    environment.systemPackages = [
      pkgs.brightnessctl
      pkgs.ddcutil
    ];
  };
}
