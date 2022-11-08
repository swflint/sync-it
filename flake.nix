{
  description = "A custom synchronization tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages = rec {

          sync-it = pkgs.rustPlatform.buildRustPackage rec {
            name = "sync-it";
            version = "2.2.0";

            src = ./.;

            cargoSha256 = "sha256-cLyk5ZZkBPjjckAyLTWK6q8Etkq1dpQmxpYB6xnd75I=";

            meta = {
              description = "A simple, customizable synchronization tool.";
              license = "GPL-3.0-or-later";
            };
          };

          devEnvironment = pkgs.mkShell {
            name = "sync-it-dev-environment";

            buildInputs = [
              pkgs.pre-commit
              pkgs.rustc
              pkgs.cargo
              pkgs.rust-analyzer
              pkgs.clippy
              pkgs.rustfmt
              pkgs.sloc
            ];
          };

        };

        defaultPackage = packages.sync-it;
        devShell = packages.devEnvironment;
      }
    );
}
