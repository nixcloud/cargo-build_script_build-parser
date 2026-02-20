{ pkgs, lib ? pkgs.lib }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "build-rs-libnix-cli";  # Adjust to match cli/Cargo.toml [package.name]
  version = "0.1.11";

  src = pkgs.fetchFromGitHub {
    owner = "nixcloud";
    repo = "build-rs-libnix";
    rev = "v${version}";  # Or commit hash if no tag
    sha256 = "sha256-DKYT8KeRh1PnzXxpYp3VrNQxGimLUNFf/RbUTeSkr2k=";
  };

  cargoHash = "sha256-gJ35ScjnwgBMux4i4oHOyxcNc3jqolMkGhoJGEA1On4=";  # Recompute for the workspace: run `nix build` and copy from error, or use `cargoSha256 = pkgs.lib.fakeSha256;` temporarily

  nativeBuildInputs = [];
  buildInputs = [];

  # Build only the CLI package (builds the lib as a dep automatically)
  cargoBuildFlags = [ "-p cli" ];  # Use the [package.name] from cli/Cargo.toml

  doCheck = false;

  meta = with lib; {
    description = "CLI for extracting rustc flags from cargo build script output";
    license = licenses.mit;
    maintainers = with maintainers; [ ];
    platforms = platforms.all;
  };
}