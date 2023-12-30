{
  description = "Punto - Another dotfiles manager";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let

        # For the rust package
        overlays = [ (import rust-overlay) ];
        rustVersion = pkgs.rust-bin.stable.latest.default;

        # pkgs = nixpkgs.legacyPackages.${system};
        pkgs = import nixpkgs { inherit system overlays; };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

      in
      {

        # Packages that we use in `nix develop`
        devShell = pkgs.mkShell {
          buildInputs = [

            # For building the project
            pkgs.cargo
            pkgs.rustc

            # LSP
            pkgs.rust-analyzer

            # Dir sync is done using underlying rsync
            pkgs.rsync
          ];

          shellHook = ''
            # Log that we're in a custom enviroment
            echo "❄️  Running custom dev enviroment "
          '';
        };

        defaultPackage = rustPlatform.buildRustPackage {
          pname = "punto";
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml
          cargoLock.lockFile = ./Cargo.lock;

          # This is needed for reqwest crate
          nativeBuildInputs = [ pkgs.pkg-config ]; # just for the host building the package
          buildInputs = [ pkgs.openssl ]; # packages needed by the consumer
          checkType = "debug";
        };

      });
}
