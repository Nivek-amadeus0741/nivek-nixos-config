{ ... }: {
  flake.modules.homeManager.mako = { pkgs, ... }: {
    services.mako = {
      enable = true;
      settings = {
        font = "JetBrainsMono Nerd Font 10.5";
        width = 380;
        height = 140;
        margin = "16,16";
        padding = "14,18";
        border-size = 2;
        border-radius = 14;
        icons = true;
        max-icon-size = 48;
        default-timeout = 5000;
        layer = "overlay";
        background-color = "#101416ee";
        text-color = "#dee3e5";
        border-color = "#82d3e177";
        progress-color = "over #82d3e144";

        "urgency=low" = {
          border-color = "#82d3e144";
          default-timeout = 3000;
        };

        "urgency=critical" = {
          border-color = "#ffb4abff";
          text-color = "#ffb4abff";
          default-timeout = 0;
        };
      };
    };
  };
}
