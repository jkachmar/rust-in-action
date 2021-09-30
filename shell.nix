# asdf
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = (with pkgs.rust.packages.stable; [
  ]) ++ (with pkgs; [
    rustup
  ]);
}
