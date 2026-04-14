{
  description = "Gitcoda - Rust + Leptos workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup
            pkgs.pkg-config
            pkgs.openssl
            pkgs.nodejs_20
          ];

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
            export NIX_LD="$(cat ${pkgs.stdenv.cc}/nix-support/dynamic-linker)"
            export NIX_LD_LIBRARY_PATH="${
              pkgs.lib.makeLibraryPath [
                pkgs.stdenv.cc.cc
                pkgs.zlib
              ]
            }"
            export NIX_ENFORCE_PURITY=0

            rustup default nightly
            rustup component add rust-analyzer
            rustup target add wasm32-unknown-unknown

            # Install build tools if not already present
            if ! command -v trunk &> /dev/null; then
              cargo install trunk
            fi

            if ! command -v ui &> /dev/null; then
              cargo install ui-cli
            fi
          '';
        };
      }
    );
}
