{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, ... }:
  flake-utils.lib.eachDefaultSystem (system: 
    let
      pkgs = import nixpkgs {
        inherit system;
      };

      inherit (pkgs) lib;
#      craneLib = crane.lib.${system};
      craneLib = (crane.mkLib nixpkgs.legacyPackages.${system}).overrideToolchain 
        fenix.packages.${system}.stable.toolchain;
      src = lib.cleanSourceWith {
        src = ./.;
        filter = path: type: (craneLib.filterCargoSources path type);
      };

      runtimeDeps = {
        inherit (craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; }) pname version src;
      };

      commonArgs = {
        inherit src;
        inherit (runtimeDeps) pname version;

        buildInputs = [
          pkgs.pkg-config
          pkgs.llvm
          pkgs.rustc.llvmPackages.llvm
        ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs // {
        pname = "runtime-deps";
      };

      runtime = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
        inherit src;
        cargoExtraArgs = "--bin task4";
        pname = "task4";
        doCheck = false;
        doNotLinkInheritedArtifacts = true;
      });

    in {
      packages = {
        default = runtime;
      };

      apps.default = flake-utils.lib.mkApp {
        drv = runtime;
      };
    });
}
