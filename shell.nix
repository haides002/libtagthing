{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # dev stuff
    cargo
    rustc
    bacon
    clippy

    # build stuff
    pkg-config
    exempi
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
}
