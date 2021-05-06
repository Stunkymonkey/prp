{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [
      pkg-config
      brotli # web
      glpk # pre
      minizip # pbfextractor
    ];
}
