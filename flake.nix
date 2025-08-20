{
  description = "The build-script-build parser flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [];
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
            #rust-bin.stable."1.87.0".default
            #packages.default
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
