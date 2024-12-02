{
  description = "Advent of Code 2024";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      inherit (nixpkgs.lib) optionals;

      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = [
          toolchain
          pkgs.rust-analyzer-unwrapped
        ];

        RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
      };
    }
  );
}
