{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    bundlers = {
      url = "github:viperML/bundlers";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ self, fenix, crane, flake-parts, advisory-db, bundlers, ... }:
    flake-parts.lib.mkFlake { inherit self inputs; } ({ withSystem, ... }: {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem = { lib, config, self', inputs', pkgs, system, ... }:
        let
          rustToolchain = fenix.packages.${system}.stable.withComponents [
            "rustc"
            "cargo"
            "rustfmt"
            "clippy"
            "rust-src"
          ];

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          commonBuildArgs = rec {
            src = craneLib.cleanCargoSource ./.;

            pname = "schwitzerflut";
            version = "v0.1.0";

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = [ ];
          };

          cargoArtifacts = craneLib.buildDepsOnly ({} // commonBuildArgs);
          clippy-check = craneLib.cargoClippy ({
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-features -- --deny warnings";
            }
            // commonBuildArgs);

          rust-fmt-check = craneLib.cargoFmt ({
              inherit (commonBuildArgs) src;
            }
            // commonBuildArgs);

          test-check = craneLib.cargoNextest ({
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            }
            // commonBuildArgs);

          doc-check = craneLib.cargoDoc ({
              inherit cargoArtifacts;
            }
            // commonBuildArgs);

          audit-check = craneLib.cargoAudit ({
              inherit (commonBuildArgs) src;
              inherit advisory-db;
            }
            // commonBuildArgs);

          server-package = craneLib.buildPackage ({
              pname = "schwitzerflut-server";
              cargoExtraFlags = "--bin schwitzerflut-server";
              inherit cargoArtifacts;
            }
            // commonBuildArgs);

          client-package = craneLib.buildPackage ({
              pname = "schwitzerflut-client";
              cargoExtraFlags = "--bin schwitzerflut-client";
              inherit cargoArtifacts;
            }
            // commonBuildArgs);
        in
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = builtins.attrValues self.checks;
          };

          packages =
            {
              server = server-package;
              client = client-package;
            };

          checks =
            {
              inherit clippy-check rust-fmt-check test-check doc-check audit-check server-package client-package;
            };

          formatter = pkgs.nixpkgs-fmt;
        };

      flake.bundlers = bundlers.bundlers;
    });

  nixConfig = {
    extra-trusted-substituters = [
      "https://cache.nixos.org/"
      "https://nix-community.cachix.org"
      "https://nix-rust-template.cachix.org"
    ];

    extra-trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "nix-rust-template.cachix.org-1:djhhKdQkilYrrV/GLYHq38Y+6hR4NAeT1NabRg6Cb7k="
    ];
  };
}
