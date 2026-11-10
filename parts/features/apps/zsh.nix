{ ... }: {
  flake.modules.nixos.zsh = { pkgs, ... }: {
    programs.zsh.enable = true;
  };

  flake.modules.homeManager.zsh = { pkgs, ... }: {
    home.packages = with pkgs; [
      eza
      bat
      zoxide
    ];

    programs.zsh = {
      enable = true;
      enableCompletion = true;
      autosuggestion.enable = true;
      syntaxHighlighting.enable = true;

      history = {
        size = 50000;
        save = 50000;
        share = true;
        ignoreDups = true;
        ignoreSpace = true;
      };

      shellAliases = {
        ls = "eza --icons";
        ll = "eza -lha --icons";
        tree = "eza --tree --icons";
        cat = "bat --paging=never";
        y = "yazi";
        lg = "lazygit";
        engine = "nivekctl";
        rebuild = "nh os switch .";
        test-os = "nh os test .";
        nix-gc = "nh clean all";
      };

      initContent = ''
        bindkey -e
        bindkey '^[[A' up-line-or-search
        bindkey '^[[B' down-line-or-search
      '';
    };

    programs.starship = {
      enable = true;
      enableZshIntegration = true;
      settings = {
        add_newline = false;
        format = "$directory$git_branch$git_status$rust$nix_shell$character";
        character = {
          success_symbol = "[➜](bold #82d3e1)";
          error_symbol = "[✗](bold #ffb4ab)";
        };
        directory = {
          style = "bold #bbc6ea";
          truncation_length = 3;
        };
        git_branch = {
          style = "bold #a0d8b3";
          format = " [$symbol$branch]($style)";
        };
      };
    };
  };
}
