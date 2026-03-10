{pkgs, ...}:
pkgs.mkShell {
  name = "element-leveldb-forensics";

  # Compile time dependencies
  nativeBuildInputs = with pkgs; [
    # Nix
    nixd
    statix
    deadnix
    alejandra

    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch
    cargo-expand

    gtk4
    libadwaita
    pkg-config
  ];

  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];

  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
