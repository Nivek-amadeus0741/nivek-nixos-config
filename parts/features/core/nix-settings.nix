{ ... }: {
  flake.modules.nixos.nix-settings = { pkgs, ... }: {
    nix.settings = {
      experimental-features = [ "nix-command" "flakes" ];
      auto-optimise-store = true;
      warn-dirty = false;
    };

    nixpkgs.config.allowUnfree = true;
  };
}
