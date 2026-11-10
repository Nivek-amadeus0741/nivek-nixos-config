{ ... }: {
  flake.modules.homeManager.editors = { pkgs, ... }: {
    # Neovim: Editor de terminal ágil, modular y extensible
    programs.neovim = {
      enable = true;
      defaultEditor = true;
      viAlias = true;
      vimAlias = true;
      vimdiffAlias = true;
      withRuby = false;
      withPython3 = false;

      extraPackages = with pkgs; [
        fd
        git
        nixd
        ripgrep
        rust-analyzer
        wl-clipboard
      ];

      plugins = with pkgs.vimPlugins; [
        nvim-treesitter.withAllGrammars
        telescope-nvim
        lualine-nvim
        which-key-nvim
        gitsigns-nvim
        nvim-lspconfig
      ];

      initLua = ''
        -- Opciones generales
        vim.g.mapleader = ' '
        vim.g.maplocalleader = ' '
        vim.opt.number = true
        vim.opt.relativenumber = true
        vim.opt.mouse = 'a'
        vim.opt.clipboard = 'unnamedplus'
        vim.opt.breakindent = true
        vim.opt.undofile = true
        vim.opt.ignorecase = true
        vim.opt.smartcase = true
        vim.opt.signcolumn = 'yes'
        vim.opt.updatetime = 250
        vim.opt.timeoutlen = 300
        vim.opt.termguicolors = true
        vim.opt.tabstop = 4
        vim.opt.shiftwidth = 4
        vim.opt.expandtab = true

        -- Configuración de Lualine
        require('lualine').setup {
          options = {
            theme = 'auto',
            component_separators = "|",
            section_separators = "",
          }
        }

        -- Configuración de Which-Key
        require('which-key').setup {}

        -- Configuración de Gitsigns
        require('gitsigns').setup {}

        -- Atajos rápidos
        vim.keymap.set('n', '<leader>ff', '<cmd>Telescope find_files<cr>', { desc = 'Buscar Archivos' })
        vim.keymap.set('n', '<leader>fg', '<cmd>Telescope live_grep<cr>', { desc = 'Buscar Texto (Grep)' })
        vim.keymap.set('n', '<leader>fb', '<cmd>Telescope buffers<cr>', { desc = 'Listar Buffers' })
        vim.keymap.set('n', '<leader>w', '<cmd>w<cr>', { desc = 'Guardar' })
        vim.keymap.set('n', '<leader>q', '<cmd>q<cr>', { desc = 'Salir' })
      '';
    };

    # VS Code: Entorno IDE completo para proyectos y depuración
    programs.vscode = {
      enable = true;
      package = pkgs.vscode;

      profiles.default = {
        extensions = with pkgs.vscode-extensions; [
          rust-lang.rust-analyzer
          jnoortheen.nix-ide
          tamasfe.even-better-toml
          pkief.material-icon-theme
        ];

        userSettings = {
          "editor.fontLigatures" = true;
          "editor.formatOnSave" = true;
          "editor.cursorBlinking" = "smooth";
          "editor.smoothScrolling" = true;
          "window.titleBarStyle" = "custom";
          "nix.enableLanguageServer" = true;
          "nix.serverPath" = "nixd";
        };
      };
    };
  };
}
