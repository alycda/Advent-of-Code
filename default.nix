{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    just
    bacon
  ];

  shellHook = ''
    rustup update
    rustc --version
  '';
}