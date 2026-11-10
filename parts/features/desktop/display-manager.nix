{ ... }: {
  flake.modules.nixos.display-manager = { pkgs, ... }: {
    services.displayManager = {
      defaultSession = "niri";
      sddm = {
        enable = true;
        wayland.enable = true;
        package = pkgs.kdePackages.sddm;
      };
    };

    services.gvfs.enable = true;
  };
}
