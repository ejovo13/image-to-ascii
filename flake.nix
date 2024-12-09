{
  description = "Convert png images to prinatble ascii characters in the terminal";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    # TODO: Move this elsewhere
    inherit (nixpkgs.stdenv) mkDerivation;
  in {
    packages.x86_64-linux = {
      default = import ./default.nix {pkgs = nixpkgs;};
    };
  };
}
