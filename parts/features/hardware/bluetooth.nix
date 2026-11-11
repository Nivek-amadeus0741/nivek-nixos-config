{ ... }: {
  flake.modules.nixos.bluetooth = { pkgs, ... }: {
    hardware.bluetooth = {
      enable = true;
      powerOnBoot = true;
      settings = {
        General = {
          Experimental = true;
          Enable = "Source,Sink,Media,Socket";
        };
      };
    };

    services.blueman.enable = true;

    environment.systemPackages = with pkgs; [
      blueman
    ];
  };

  flake.modules.homeManager.bluetooth = { pkgs, ... }: {
    home.packages = with pkgs; [
      blueman
    ];
  };
}
