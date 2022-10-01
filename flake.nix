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
            version = "2.0.0";

            src = ./.;

            cargoSha256 = "B8N7fgi8RTXXzVYfqzbCB3ov6mDIJgFm6HEg3gCuVGA=";

            meta = {
              description = "A simple, customizable synchronization tool.";
              license = "GPL3+";
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
