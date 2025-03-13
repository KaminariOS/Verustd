{
  description = "Atmosphere";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      # inputs.nixpkgs.follows = "mars-std/nixpkgs";
      # inputs.flake-utils.follows = "mars-std/flake-utils";
      # inputs.flake-compat.follows = "mars-std/flake-compat";
    };
    verus = {
      url = "github:KaminariOS/verus/flakey";
      # inputs.nixpkgs.follows = "mars-std/nixpkgs";
      # inputs.flake-utils.follows = "mars-std/flake-utils";
      # inputs.flake-compat.follows = "mars-std/flake-compat";
      inputs.crane.follows = "crane";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, verus, ... }: let
    supportedSystems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
  in flake-utils.lib.eachSystem supportedSystems (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [
        rust-overlay.overlays.default
      ];
    };
    inherit (pkgs) lib;
    x86Pkgs = nixpkgs.legacyPackages.x86_64-linux;
    x86Tools = pkgs.pkgsCross.gnu64;

    pinnedRust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    # rustPlatform = pkgs.makeRustPlatform {
    #   rustc = pinnedRust;
    #   cargo = pinnedRust;
    # };

    # craneLib = (crane.mkLib pkgs).overrideToolchain pinnedRust;

    pinnedVerus = verus.packages.${system};

    mkShell = pkgs.mkShell.override {
      stdenv = pkgs.llvmPackages_14.stdenv;
    };

    mkCargoShim = sub: pkgs.writeShellScriptBin "cargo-${sub}" ''
      shift 1
      exec atmo ${sub} "$@"
    '';
  in {
    devShell = mkShell {
      nativeBuildInputs = [
        pinnedVerus.verus-alloc
        pinnedVerus.line-count
        pinnedVerus.vargo
        # pinnedRust
        

        # pkgs.mars-research.mars-tools
      ] ++ (with pkgs; [
            rustup
    #         rust-bin.nightly."2024-11-01".default.override {
    #     extensions = ["rustc-dev" "rust-src" "rust-analyzer-preview" ];
    #   targets = [ "x86_64-unknown-linux-gnu" ];
    # }
        # llvmPackages_14.bintools
        # llvmPackages_14.llvm
        #
        # cargo-expand cargo-outdated cargo-edit
        #
        # pkg-config
        #
        # x86Pkgs.grub2
        # xorriso
        #
        # util-linuxMinimal
        #
        # nasm
        #
        # gdb
        #
        # qemu
        #
        # editorconfig-checker
        #
        # jq

        # (python3.withPackages (py: with py; [ z3 ]))
        #
        # (pkgs.writeShellScriptBin "x86_64.ld" ''
        #   exec ${x86Tools.buildPackages.bintools}/bin/${x86Tools.stdenv.cc.targetPrefix}ld "$@"
        # '')
        #
        # (pkgs.writeShellScriptBin "atmo" ''
        #   set -euo pipefail
        #   metadata=$(cargo metadata --format-version 1)
        #   root=$(echo $metadata | jq -r '.workspace_root')
        #   target_dir=$(echo $metadata | jq -r '.target_directory')
        #   pushd "$root/build-tool" >/dev/null
        #   cargo build --quiet
        #
        #   "$target_dir/debug/atmo" "$@"
        # '')
        #
        # (mkCargoShim "verify")
        # (mkCargoShim "gdb")
      ]);

      # buildInputs = [ pkgs.openssl ];

      # RUSTC_BOOTSTRAP = "1";

      # Used by build-tool
      # GRUB_X86_MODULES = "${x86Pkgs.grub2}/lib/grub/i386-pc";

      # For vstd_build
      #RUST_SYSROOT = pinnedRust;

      # For rust_verify
      shellHook = ''
        export LD_LIBRARY_PATH="${pinnedRust}/lib";
      '' + lib.optionalString pkgs.stdenv.isDarwin ''
        export DYLD_LIBRARY_PATH="${pinnedRust}/lib";
      '';
    };
  });
}
