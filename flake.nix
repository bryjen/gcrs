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
            pkgs.llvmPackages.lld

            # TLS / HTTP (needed by reqwest / axum / tower-http)
            pkgs.openssl
            pkgs.openssl.dev

            # Node (Tailwind CSS CLI runs via Node)
            pkgs.nodejs_20

            # cargo-leptos calls the Tailwind binary; make sure it is on PATH
            pkgs.tailwindcss_4
          ];

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"

            # Dynamic linker shim — lets nix-built binaries run on non-NixOS
            export NIX_LD="$(cat ${pkgs.stdenv.cc}/nix-support/dynamic-linker)"
            export NIX_LD_LIBRARY_PATH="${
              pkgs.lib.makeLibraryPath [
                pkgs.stdenv.cc.cc
                pkgs.zlib
                pkgs.openssl
              ]
            }"
            export NIX_ENFORCE_PURITY=0

            # OpenSSL for cargo build (native-tls / openssl-sys)
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"

            rustup default nightly
            rustup component add rust-analyzer rust-src
            rustup target add wasm32-unknown-unknown

            # cargo-leptos replaces trunk
            if ! command -v cargo-leptos &> /dev/null; then
              echo "Installing cargo-leptos..."
              cargo install cargo-leptos --locked
            fi

            if ! command -v ui &> /dev/null; then
              cargo install ui-cli
            fi
          '';
        };
      }
    );
}
