{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              openssl
            ];
            packages = [
              toolchain

              # We want the unwrapped version, "rust-analyzer" (wrapped) comes with nixpkgs' toolchain
              pkgs.rust-analyzer-unwrapped
            ];

            shellHook = "
              export RUST_SRC_PATH=${toolchain}/lib/rustlib/src/rust/library
            ";
          };
        }
    );
}
