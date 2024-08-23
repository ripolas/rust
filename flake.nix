{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:let pkgs = nixpkgs.legacyPackages."x86_64-linux"; in { 
    
    packages.x86_64-linux.default = (pkgs.makeRustPlatform {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        }).buildRustPackage {
          pname = "astronomija";
          version = "0.1.0";
          src = ./.;
          cargoSha256 = "sha256-sn+Z/ceuAOshl2tsVO0MZiAg2ua0+8eJjtOTpbZ51vk=";
        };
    };
}
