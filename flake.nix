{
  description = "Flake for building and testing nu_plugin_units";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          pkgs,
          lib,
          self',
          ...
        }:
        {
          treefmt = {
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
          };

          packages.default = pkgs.nushell-plugin-units.overrideAttrs (
            final: prev: {
              version = "0.1.8"; # used by versionCheckHook, keep in sync with Cargo.toml
              src = lib.fileset.toSource {
                root = ./.;
                fileset = lib.fileset.unions [
                  ./Cargo.toml
                  ./Cargo.lock
                  ./src
                ];
              };
              cargoHash = "";
              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                inherit (final) src;
                hash = "sha256-RGjRRWIIwvnrt8uYK/n2/htIp59Pf+uXqulfamC1KOk=";
              };
              meta = prev.meta // {
                broken = false;
              };
            }
          );

          # the nushell build we are currently supporting. Should loosely
          # follow the upstream nixpkgs build to ensure compatibility with the
          # latest version of nushell. Used for load-check.
          packages.nushell = pkgs.nushell.overrideAttrs rec {
            version = "0.112.2";

            src = pkgs.fetchFromGitHub {
              owner = "nushell";
              repo = "nushell";
              tag = "${version}";
              hash = "sha256-wc7mfbwkJO5gq9mwsiTVx74+btqU6Ox8tPhnXkfmXRU=";
            };

            checkPhase = "";
            doCheck = false;

            cargoDepsName = "nushell-${version}";
            cargoHash = "";

            cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
              inherit src;
              hash = "sha256-KBDgICbdYcqgMLtUXWQsMPe1fO7zT4NcavAyS2i0cDc=";
            };
          };

          checks.default =
            pkgs.runCommand "test-load-${self'.packages.default.name}"
              {
                nativeBuildInputs = [ self'.packages.nushell ];
              }
              ''
                touch $out
                nu --plugins ${lib.getExe self'.packages.default} --plugin-config $out -c 'units --help'
              '';

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              self'.packages.nushell
            ];
          };
        };
    };
}
