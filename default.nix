{pkgs, lib} :

pkgs.rustPlatform.buildRustPackage {
    pname = "cargo-build_script_build-parser";
    version = "0.1.0";

    src = ./.;

    cargoLock = {
        lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ ];
    buildInputs = [ ];

    meta = with lib; {
        description = "A tool to extract rustc flags from cargo build script output";
        license = licenses.mit;
        maintainers = with maintainers; [ ];
        platforms = platforms.all;
    };
}