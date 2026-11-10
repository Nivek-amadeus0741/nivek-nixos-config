{ config, lib, inputs, ... }:
let
  inherit (inputs) nixpkgs home-manager stylix;

  # Colectar todos los módulos dendríticos automáticamente
  nixosModules = builtins.attrValues config.flake.modules.nixos;
  homeManagerModules = builtins.attrValues config.flake.modules.homeManager;

  # Inyectar los binarios de Rust (nivek-core y nivek-cli) en nixpkgs
  rustOverlay = final: prev: {
    nivek-core = config.flake.packages.x86_64-linux.nivek-core;
    nivek-cli = config.flake.packages.x86_64-linux.nivek-cli;
  };

  commonNixosModules = [
    {
      nixpkgs.overlays = [ rustOverlay ];
    }
    stylix.nixosModules.stylix
    home-manager.nixosModules.home-manager
  ];

  commonHomeManagerSettings = {
    useGlobalPkgs = true;
    useUserPackages = true;
    backupFileExtension = "hm-backup";
    sharedModules = homeManagerModules;
  };

  mkHost = { hostname, hostModule }:
    nixpkgs.lib.nixosSystem {
      specialArgs = { inherit inputs; };
      modules =
        commonNixosModules
        ++ nixosModules
        ++ [
          hostModule
          {
            home-manager = commonHomeManagerSettings // {
              extraSpecialArgs = { inherit inputs; };
              users.nivek = { ... }: {
                home.stateVersion = "24.11";
                home.username = "nivek";
                home.homeDirectory = "/home/nivek";
              };
            };
          }
        ];
    };
in
{
  flake.nixosConfigurations = {
    nixos-lab = mkHost {
      hostname = "nixos-lab";
      hostModule = ./_hosts/default;
    };
  };
}
