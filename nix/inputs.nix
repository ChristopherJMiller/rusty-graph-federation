{ pkgs, ... }:

let
  buildInputs = with pkgs; [
    nodejs
    yarn
    dive
    just
    minikube
  ];
in
{
  inherit buildInputs;
}