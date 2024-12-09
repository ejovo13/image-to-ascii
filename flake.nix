{
  description = "Convert png images to prinatble ascii characters in the terminal";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
  in {
    packages.x86_64-linux = {
      default = import ./default.nix {
        pkgs = import nixpkgs {
          system = "x86_64-linux";
        };
      };
    };
  };
}
