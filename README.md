# cargo-build_script_build-parser

A command-line utility that extracts `--cfg` and `--check-cfg` flags from `cargo:` lines generated by a Rust `build.rs` script.

This is useful when integrating crates like `libc`, which emit configuration flags during their build process. 

    https://github.com/nixcloud/cargo-build_script_build-parser

<https://doc.rust-lang.org/cargo/reference/build-scripts.html>

- Outputs all flags as a single command-line string
- Simple command-line interface
- Tests included
- Distributed as a [Nix Flake](#flake-usage)

## 🔧 Supported Features

    cargo:cargo-cfg
    cargo:cargo-check-cfg
    cargo:rustc-env
    cargo:warning
    cargo:error
    cargo:rustc-link-search
    cargo:rustc-link-lib
    cargo:version_number
    cargo:include
    cargo:root
    cargo:conf
    cargo:version_number
    cargo:static

## 🔧 Experimental Features

## 🔧 Intentionally Ignored Features

    cargo:rerun-if-changed
    cargo:rerun-if-env-changed
    cargo:rerun-if-changed-bin
    cargo:rerun-if-changed-glob
    cargo:rerun-if-changed-dir
    cargo:rerun-if-changed-recursive
    cargo:rerun-if-changed-env
    cargo:lib_dir

## 🔧 Missing Features

    cargo:rustc-flags
    cargo:rustc-cdylib-link-arg
    cargo:rustc-bin-link-arg
    cargo:rustc-link-arg-bin

### Extract rustc-arguments

    nix run .#default -- test/output1 rustc-arguments

Output:

    --cfg 'freebsd11' --cfg 'libc_const_extern_fn' --check-cfg 'cfg(espidf_time32)' --check-cfg 'cfg(target_arch,values("mips64r6"))'

### Extract environment-variables

    nix run .#default -- test/output1 environment-variables

Output:

    VAR=VALUE
    VAR2=

### Extract rustc-propagated-arguments

    nix run .#default -- test/output3 rustc-propagated-arguments

Output:

    warning: In file included from /nix/store/x4cz3spvw0bwwz5sjsdn2qm4f89rcryn-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
    warning: from /nix/store/x4cz3spvw0bwwz5sjsdn2qm4f89rcryn-glibc-2.40-66-dev/include/stdio.h:28,
    warning: from sqlite3/sqlite3.c:14884:
    warning: /nix/store/x4cz3spvw0bwwz5sjsdn2qm4f89rcryn-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
    warning: 422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
    warning: |    ^~~~~~~
    -L 'native=${rust-embed-8_6_0-50d2bdadc507cf36}'

### Writing these files

    nix run .#default -- test/output1 --out-path out/ write-results

Output

    warning: This is a custom build warning from build.rs!
    Successfully created files for nix to process from build.rs output in 'out/'

# Hacking

When traditional cargo executes build.rs scripts it stores the output of each in this folder structure:

    du -a target/ | grep output | grep -v '.finger'
    4       target/debug/build/openssl-sys-bf6c2c38618f44c9/root-output
    8       target/debug/build/openssl-sys-bf6c2c38618f44c9/output
    4       target/debug/build/parking_lot_core-7bae69a33df63e20/root-output
    4       target/debug/build/parking_lot_core-7bae69a33df63e20/output
    4       target/debug/build/generic-array-39173d8ec8bd3f99/root-output
    4       target/debug/build/generic-array-39173d8ec8bd3f99/output
    4       target/debug/build/im-rc-723454c6ae50b5e6/root-output
    4       target/debug/build/im-rc-723454c6ae50b5e6/output
    4       target/debug/build/pulldown-cmark-1b80d253c6640a7e/root-output
    ...

So look at the *./output files and filter them with:

    cat target/debug/build/openssl-sys-bf6c2c38618f44c9/output | grep '^cargo:'

# 🚀 Installation

This project is distributed as a Nix Flake.

```nix
{
  cargo-build_script_build-parser.url = "github:nixcloud/cargo-build_script_build-parser";

  outputs = { self, cargo-build_script_build-parser, ... }: {
    packages.x86_64-linux.cargo-build_script_build-parser = cargo-build_script_build-parser.packages.x86_64-linux.default;
  };
}
```

# 🧪 Testing

To verify functionality, run:

    cargo test

The tests are written directly in main.rs.

# 📄 License

MIT © 2025 nixcloud

# 🙏 Acknowledgements

This utility was inspired by how libc and similar crates communicate configuration to cargo and downstream consumers. It simplifies extracting those configurations in reproducible or cross-compiled environments.
