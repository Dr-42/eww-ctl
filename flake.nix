
{
  description = "eww-ctl: Control center for Eww widgets";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # 1. Build-time tools (compilers, pkg-config to find libs)
        nativeBuildInputs = with pkgs; [
          pkg-config
          rust-bin.stable.latest.default # Uses the rust-overlay for latest stable
        ];

        # 2. Runtime libraries (ALSA for sound, OpenCL for ocl crate)
        buildInputs = with pkgs; [
          alsa-lib
          ocl-icd
          opencl-headers
        ];
      in
      {
        # The environment you enter with 'nix develop'
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # Vital for the 'ocl' crate to find the OpenCL loader at runtime
          LD_LIBRARY_PATH = "${pkgs.ocl-icd}/lib";
        };
      }
    );
}
