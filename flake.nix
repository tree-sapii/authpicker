{
  description = "Strict jailed devshell with explicit read-only mounts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    jail-nix.url = "sourcehut:~alexdavid/jail.nix";
  };

  outputs = { self, nixpkgs, jail-nix }: 
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      jail = jail-nix.lib.init pkgs;

    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          # Wrap the bash shell with our specific rules
          (jail "fishjail" pkgs.fish (with jail.combinators; [
            no-new-session                 # Required for TUI programs (vim, fzf, etc.) to work
            mount-cwd
            network
            (readonly "/nix/store")
            (readonly "${builtins.getEnv "HOME"}/.config/")  # Mount specific directory as read-only
            (readwrite "${builtins.getEnv "HOME"}/.local/")  # Mount specific directory as read-only
            
            ( add-pkg-deps (with pkgs; [eza zoxide zellij fzf starship neovim devenv git ]) )
          ]))
        ];

        shellHook = ''
          echo "🔒 Spawning strictly isolated devshell..."
          exec fishjail
        '';
      };
    };
}
