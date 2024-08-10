{
  description = "Gateway Service";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        node-modules = pkgs.mkYarnPackage {
          name = "node-modules";
          src = ./.;
        };

        service = pkgs.stdenv.mkDerivation {
            name = "gateway";
            src = ./.;
            buildInputs = [pkgs.yarn node-modules];
            buildPhase = ''
              ln -s ${node-modules}/libexec/gateway/node_modules node_modules
              ${pkgs.yarn}/bin/yarn build
            '';
            installPhase =  ''
              mkdir -p $out/app
              mv build/src/* $out/app
            '';
        };

        dockerImage = pkgs.dockerTools.streamLayeredImage {
          name = "rusty-graph-gateway";
          tag = "latest";

          contents = [ pkgs.nodejs service ];
          config = {
            Cmd = [ "${pkgs.nodejs}/bin/node" "${service}/app/main.js" ];
          };
        };
      in 
        {
          packages = {
            inherit service node-modules dockerImage ;
            default = service;
          };
        }
    );
}