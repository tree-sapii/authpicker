{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  # packages = [ pkgs.rust_analyzer ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";
  # See full reference at https://devenv.sh/reference/options/
}
