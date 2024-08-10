{ pkgs ? import <nixpkgs> {} }:
let
  
in
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      pkg-config
      openssl
      nodejs
      yarn
      dive
    ];
  }