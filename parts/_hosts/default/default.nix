{ ... }: {
  imports = [
    ./hardware-configuration.nix
  ];

  networking.hostName = "nixos-lab";
  time.timeZone = "America/Bogota";
  i18n.defaultLocale = "en_US.UTF-8";

  # Activar el motor reactivo de Nivek en Rust
  services.nivek-engine.enable = true;

  system.stateVersion = "24.11";
}
