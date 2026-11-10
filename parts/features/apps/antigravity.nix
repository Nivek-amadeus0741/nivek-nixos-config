{ ... }: {
  flake.modules.homeManager.antigravity = { pkgs, ... }:
    let
      # Wrapper de conveniencia para Antigravity CLI si está en ~/.local/bin/agy o ~/.gemini/antigravity-cli/bin/agy
      agyWrapper = pkgs.writeShellScriptBin "agy" ''
        if [ -x "$HOME/.local/bin/agy" ]; then
          exec "$HOME/.local/bin/agy" "$@"
        elif [ -x "$HOME/.gemini/antigravity-cli/bin/agy" ]; then
          exec "$HOME/.gemini/antigravity-cli/bin/agy" "$@"
        else
          echo "Antigravity CLI (agy) no encontrado en ~/.local/bin ni ~/.gemini/antigravity-cli/bin"
          exit 1
        fi
      '';
    in {
      home.packages = [
        agyWrapper
      ];

      home.sessionPath = [
        "$HOME/.local/bin"
        "$HOME/.gemini/antigravity-cli/bin"
      ];

      home.shellAliases = {
        antigravity = "agy";
      };
    };
}
