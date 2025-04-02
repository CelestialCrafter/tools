{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    rec {
      packages.x86_64-linux.fetch = pkgs.buildGoModule {
        name = "fetch";
        version = "0.1.0";
        src = pkgs.lib.cleanSource ./.;

        vendorHash = "sha256-Phi48RRLlPNrZdcNhd0cTH1UedAwBlMsfsV7HK0W4IQ=";

        meta = {
          description = "a simple fetch script";
          homepage = "https://github.com/CelestialCrafter/tools";
          license = pkgs.lib.licenses.mpl20;
        };
      };

      packages.x86_64-linux.lyricizer = pkgs.rustPlatform.buildRustPackage {
        name = "lyricizer";
        version = "0.1.0";
        src = pkgs.lib.cleanSource ./.;

        useFetchCargoVendor = true;
        cargoHash = "sha256-VguFQMi2FEBtEAQj9Ak4UuWUXtjAQk46liAZ29zijFk=";
        buildInputs = with pkgs; [
          pkg-config
          openssl
        ];

        meta = {
          description = "downloads lyrics for songs in mpd";
          homepage = "https://github.com/CelestialCrafter/tools";
          license = pkgs.lib.licenses.mpl20;
        };
      };

      devShells.x86_64-linux.fetch = pkgs.mkShell {
        packages = [ pkgs.go ];
      };

      devShells.x86_64-linux.lyricizer = pkgs.mkShell {
        packages =
          with pkgs;
          [
            rustc
            cargo
          ]
          ++ packages.x86_64-linux.lyricizer.buildInputs;
      };
    };
}
