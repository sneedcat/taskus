{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix, }: utils.lib.eachDefaultSystem (system: let 
    pkgs = nixpkgs.legacyPackages.${system};
    rust = fenix.packages.${system};
    lib = pkgs.lib;
  in {
    devShell = pkgs.mkShell {
      buildInputs = with pkgs; with llvmPackages; [
        clang rust.complete.toolchain pkg-config sqlite libclang.lib
      ];
      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      RUST_BACKTRACE = 1;
      RUSTFLAGS = "-C target-cpu=native";
    };
  });
}

