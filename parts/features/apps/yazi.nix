{ ... }: {
  flake.modules.homeManager.yazi = { pkgs, ... }: {
    home.packages = with pkgs; [
      ffmpegthumbnailer
      unar
      jq
      poppler
      fd
      ripgrep
      fzf
      zoxide
      imagemagick
    ];

    programs.yazi = {
      enable = true;
      enableZshIntegration = true;
      shellWrapperName = "y";

      settings = {
        mgr = {
          show_hidden = true;
          sort_by = "natural";
          sort_reverse = false;
          sort_dir_first = true;
          linemode = "size";
        };
        preview = {
          tab_size = 2;
          max_width = 1200;
          max_height = 1200;
        };
      };
    };

    # Configuración de Yazi como explorador de archivos predeterminado del sistema
    xdg.mimeApps = {
      enable = true;
      defaultApplications = {
        "inode/directory" = [ "yazi.desktop" ];
      };
    };

    xdg.desktopEntries.yazi = {
      name = "Yazi";
      genericName = "Explorador de Archivos";
      comment = "Explorador de archivos asíncrono ultra rápido en terminal";
      icon = "system-file-manager";
      exec = "${pkgs.kitty}/bin/kitty --class yazi -e yazi %u";
      terminal = false;
      categories = [ "System" "FileTools" "FileManager" ];
      mimeType = [ "inode/directory" ];
    };
  };
}
