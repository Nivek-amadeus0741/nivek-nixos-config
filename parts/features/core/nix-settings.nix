{ ... }: {
  flake.modules.nixos.nix-settings = { pkgs, ... }: {
    nix.settings = {
      experimental-features = [ "nix-command" "flakes" ];
      auto-optimise-store = true;
      warn-dirty = false;
    };

    nixpkgs.config.allowUnfree = true;

    # Habilita nix-ld para ejecutar binarios ELF dinámicos precompilados (como Antigravity CLI 'agy')
    programs.nix-ld.enable = true;
    programs.nix-ld.libraries = with pkgs; [
      stdenv.cc.cc
      zlib
      fuse3
      icu
      openssl
      curl
      glib
    ];
  };
}
