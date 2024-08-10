{ pkgs, ... }:

let
  buildInputs = with pkgs; [
    pkg-config
    openssl
    nodejs
    yarn
  ];
in
{
  inherit buildInputs;
}