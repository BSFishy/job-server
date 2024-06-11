let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rust-analyzer
  ];
}
