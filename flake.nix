{
  description = "Convert png images to prinatble ascii characters in the terminal";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = import nixpkgs {system = "x86_64-linux";};
  in {
    packages.x86_64-linux = {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "image-to-ascii-rust";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = [pkgs.pkg-config];
      };
    };

    app.x86_64-linux.default = {
      type = "app";
      program = "${self.packages.x86_64-linux.default}/bin/img2ascii";
    };
  };
}
