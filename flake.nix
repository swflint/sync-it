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
            version = "1.3.0";

            src = ./.;

            cargoSha256 = "sJD1YjjIRxDNKy0KOh6nTGT3um6ZIIqd0ACn6l1tjwA=";

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
