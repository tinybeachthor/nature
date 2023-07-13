{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          git
          rust-analyzer
          cargo-edit
          cargo-machete

          rust-bin.stable.latest.default
        ];
      };
    });
}
