{
  description = "A custom synchronization tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages = rec {

          sync-it = pkgs.rustPlatform.buildRustPackage rec {
            name = "sync-it";
            version = "0.2.0";

            src = ./.;

            cargoSha256 = "b3ma+72TI1jHcnllAswCh9bNeatQuM5jQlu922dBVm4=";

            meta = with pkgs.stdenv.lib; {
              description = "A simple, customizable synchronization tool.";
              license = licenses.gpl3Plus;
              maintainers = with maintainers; [ swflint ];
            };
          };

          devEnvironment = pkgs.mkShell {
            name = "sync-it-dev-environment";

            buildInputs = [
              pkgs.pre-commit
              pkgs.rustc
              pkgs.cargo
              pkgs.rls
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
