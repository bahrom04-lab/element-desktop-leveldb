{
  pkgs ?
    let
      lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
      nixpkgs = fetchTarball {
        url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
        sha256 = lock.narHash;
      };
    in
    import nixpkgs { overlays = [ ]; },
  ...
}:
let
  # Helpful nix function
  lib = pkgs.lib;
  # getLibFolder = pkg: "${pkg}/lib"; # uncomment for LDs

  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  # Your govnocodes
  src = pkgs.lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Use this if you have dependencies from git instead
    # of crates.io in your Cargo.toml
    # outputHashes = {
    #   # Sha256 of the git repository, doesn't matter if it's monorepo
    #   "example-0.1.0" = "sha256-80EwvwMPY+rYyti8DMG4hGEpz/8Pya5TGjsbOBF0P0c=";
    # };
  };

  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    gtk4
    libadwaita
    pkg-config
  ];

  buildInputs = with pkgs; [

  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  meta = with lib; {
    homepage = manifest.homepage;
    description = manifest.description;
    # https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix
    license = with licenses; [ mit ];
    platforms = platforms.linux;
    maintainers = with maintainers; [ bahrom04 ];
  };
}
