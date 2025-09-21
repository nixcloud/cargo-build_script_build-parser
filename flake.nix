{
  description = "The build-script-build parser flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    fenix.url        = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
          overlays = [
              fenix.overlay
            ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      rec {
        packages.default = pkgs.callPackage ./default.nix {};
        apps.default = flake-utils.lib.mkApp {
          drv = packages.default;
        };
        devShells.default = mkShell {
          buildInputs = [
            fenix.packages.${system}.stable.rustc
              fenix.packages.${system}.stable.cargo
              fenix.packages.${system}.stable.rust-src
              fenix.packages.${system}.stable.rustfmt
              fenix.packages.${system}.stable.clippy
          ];
        };
      }
    ) // {
      # 🔁 expose overlay for importing into other flakes
      overlay = final: prev: {
        parse-build = prev.callPackage ./default.nix {};
      };
    };
}
