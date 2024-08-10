{ pkgs, ... }:

let
  buildInputs = with pkgs; [
    nodejs
    yarn
    dive
    just
  ];
in
{
  inherit buildInputs;
}