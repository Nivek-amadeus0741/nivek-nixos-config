{ ... }: {
  flake.modules.homeManager.zen-browser = { pkgs, inputs, ... }: {
    home.packages = [
      inputs.zen-browser.packages."${pkgs.stdenv.hostPlatform.system}".default
    ];

    home.sessionVariables = {
      MOZ_ENABLE_WAYLAND = "1";
    };
  };
}
