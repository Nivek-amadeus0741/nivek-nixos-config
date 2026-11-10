{ inputs, ... }: {
  perSystem = { pkgs, ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;
      src = craneLib.cleanCargoSource ../rust-engine;

      commonArgs = {
        inherit src;
        strictDeps = true;
        buildInputs = [ ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      nivekCore = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
        pname = "nivek-core";
        cargoExtraArgs = "-p nivek-core";
      });

      nivekCli = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
        pname = "nivek-cli";
        cargoExtraArgs = "-p nivek-cli";
      });
    in
    {
      packages = {
        default = nivekCore;
        nivek-core = nivekCore;
        nivek-cli = nivekCli;
      };

      devShells.default = craneLib.devShell {
        inherit (commonArgs) buildInputs nativeBuildInputs;
        packages = [
          pkgs.rust-analyzer
          pkgs.clippy
          pkgs.rustfmt
        ];
      };
    };
}
