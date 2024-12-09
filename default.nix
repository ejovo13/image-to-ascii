{ pkgs ? import <nixpkgs> {}}:
let
    inherit (pkgs.stdenv) mkDerivation;
in
mkDerivation {
    pname = "image-to-ascii-rust"
    buildInputs =  "";
}
