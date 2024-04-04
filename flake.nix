{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs.lib) optionals;

        craneLib = crane.lib.${system};

        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = [ ] ++ optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        oklabby = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in
      {
        checks = {
          fmt = craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });
          doc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });
          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --all-features -- -Dclippy::all -Dwarnings";
            }
          );
        };
        packages.default = oklabby;
        apps.default = flake-utils.lib.mkApp { drv = oklabby; };
        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    )
    // {
      overlays.default = _: prev: { oklabby = self.packages.${prev.system}.default; };
    };
}
