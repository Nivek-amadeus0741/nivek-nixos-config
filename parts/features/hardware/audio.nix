{ ... }: {
  flake.modules.nixos.audio = { pkgs, ... }: {
    # Soporte de PipeWire nativo y baja latencia
    security.rtkit.enable = true;
    services.pipewire = {
      enable = true;
      alsa.enable = true;
      alsa.support32Bit = true;
      pulse.enable = true;
      wireplumber.enable = true;
    };

    environment.systemPackages = [
      pkgs.wireplumber
      pkgs.pavucontrol
      pkgs.pulsemixer
    ];
  };
}
