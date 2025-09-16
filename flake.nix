{
  description = "Dev shell for libtagthing";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    lib = pkgs.lib;
  in {
    devShell.x86_64-linux = pkgs.mkShellNoCC {
      buildInputs = with pkgs; [
        cargo
        rustc
        gcc

        pkg-config

        bacon
        rustfmt
        clippy
      ];

      nativeBuildInputs = with pkgs; [
        pkg-config
      ];

      LD_LIBRARY_PATH =
        lib.makeLibraryPath [
        ];
    };
  };
}
