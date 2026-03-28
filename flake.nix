{
  description = "Build AuthPicker";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      authpicker = pkgs.rustPlatform.buildRustPackage {
          pname = "AuthPicker";
          version = "0.1.0";
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
    in {
      packages.${system}.default = pkgs.callPackage authpicker { };
    };
}
