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

          # sync-it = pkgs.rustPlatform.buildRustPackage rec {
          #   name = "sync-it";
          #   version = "0.0.1";

          #   cargoSha256 = "XXX";

          #   meta = with pkgs.stdenv.lib; {
          #     description = "A simple, customizable synchronization tool.";
          #     license = licenses.gpl3OrLater;
          #     maintainers = with maintainers; [ swflint ];
          #   };
          # };

          devEnvironment = pkgs.mkShell {
            name = "sync-it-dev-environment";

            buildInputs = [
              pkgs.pre-commit
              pkgs.rustc
              pkgs.cargo
              pkgs.rls
              pkgs.clippy
            ];
          };

        };

        devShell = packages.devEnvironment;
      }
    );
}
