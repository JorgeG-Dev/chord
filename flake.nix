{
  description = "Rust Devshell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
        "aarch64-linux"
      ];

      perSystem =
        { pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              rustfmt
              rust-analyzer
              clippy

              # Used for final linking of output
              gcc

              # This is used for any crates that have
              # dependency on system libraries
              pkg-config

              # Any crates that have a dependency on a system library,
              # add it here. Ex: glib

            ];

            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        };
    };

}
