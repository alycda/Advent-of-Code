{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    just
    bacon
  ];

  shellHook = ''
    rustup update
    rustc --version
  '';
}