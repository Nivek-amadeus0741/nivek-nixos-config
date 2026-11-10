{ ... }: {
  flake.modules.homeManager.git = { pkgs, ... }: {
    home.packages = with pkgs; [
      github-cli
      lazygit
    ];

    programs.git = {
      enable = true;

      settings = {
        user = {
          name = "Nivek_Amadeus";
          email = "kevinsans741@gmail.com";
        };

        alias = {
          st = "status";
          co = "checkout";
          br = "branch";
          ci = "commit";
          cm = "commit -m";
          ca = "commit --amend";
          cp = "cherry-pick";
          pf = "push --force-with-lease";
          lg = "log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit";
        };

        init.defaultBranch = "main";
        pull.rebase = true;
        rebase.autoStash = true;
        merge.conflictStyle = "zdiff3";
        diff.colorMoved = "default";
        push.autoSetupRemote = true;
      };

      ignores = [
        ".direnv"
        "result"
        "result-*"
        ".envrc"
        "*.swp"
        "*~"
        ".DS_Store"
        ".idea"
        ".vscode"
      ];
    };

    programs.delta = {
      enable = true;
      enableGitIntegration = true;
      options = {
        navigate = true;
        line-numbers = true;
        side-by-side = false;
      };
    };

    programs.lazygit = {
      enable = true;
      settings = {
        gui = {
          theme = {
            activeBorderColor = [ "#82d3e1" "bold" ];
            inactiveBorderColor = [ "#738a92" ];
            selectedLineBgColor = [ "#1f2b30" ];
          };
          showIcons = true;
        };
        git = {
          paging = {
            colorArg = "always";
            pager = "delta --dark --paging=never";
          };
        };
      };
    };
  };
}
