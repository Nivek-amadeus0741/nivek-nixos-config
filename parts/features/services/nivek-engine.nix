{ ... }: {
  flake.modules.nixos.nivek-engine = { config, lib, pkgs, ... }: {
    options.services.nivek-engine = {
      enable = lib.mkEnableOption "Motor Reactivo de Sistemas Nivek (Rust Tokio Daemon)";
    };

    config = lib.mkIf config.services.nivek-engine.enable {
      environment.systemPackages = [
        pkgs.nivek-cli
        pkgs.nivek-core
      ];

      systemd.user.services.nivek-engine = {
        description = "Nivek Reactive OS Core Engine";
        wantedBy = [ "default.target" ];
        after = [ "network.target" ];
        serviceConfig = {
          ExecStart = "${pkgs.nivek-core}/bin/nivek-core";
          Restart = "on-failure";
          RestartSec = 2;
        };
      };
    };
  };

  flake.modules.homeManager.nivek-engine = { ... }: { };
}
