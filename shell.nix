# vim: set expandtab tabstop=2 softtabstop=2 shiftwidth=2:

let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/refs/tags/24.05.tar.gz")) {};

in pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.clippy
    pkgs.rust-analyzer
    pkgs.python311
    pkgs.python311Packages.pip
    pkgs.nodejs_22
  ];
  shellHook = ''
    python -m venv .venv
    source .venv/bin/activate
    pip install posting
    npm install hello-world-server
  '';
}
