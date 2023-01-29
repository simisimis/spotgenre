{
  description = "Genre analyzer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };

        rustPackage = rustPlatform.buildRustPackage {
          name = "spotgenre";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            openssl
            pkg-config
            just
            zsh
          ];
          shellHook = ''
            cat <<EOF
            Welcome to the 🎷spotgenre🪘 development shell.

            $(just -l |sed 's/^Available recipes:/The following `just` recipes are available:/')
            EOF
            . ./.env
            exec zsh
          '';
        };

        packages = {
          rustPackage = rustPackage;
        };
      }
    );
}
