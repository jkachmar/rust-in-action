# asdf
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = (with pkgs.rust.packages.stable; [
  ]) ++ (with pkgs; [
    # Rust toolchain management.
    rustup
    # `cargo-edit` dependency.
    pkgconfig
    openssl
  ]);
}
