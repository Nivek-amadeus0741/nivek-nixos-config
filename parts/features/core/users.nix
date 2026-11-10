{ ... }: {
  flake.modules.nixos.users = { pkgs, ... }: {
    users.users.nivek = {
      isNormalUser = true;
      description = "Nivek";
      extraGroups = [ "networkmanager" "wheel" "video" "audio" "input" ];
      shell = pkgs.zsh;
    };
  };
}
