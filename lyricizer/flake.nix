{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      buildInputs = with pkgs; [
        pkg-config
        openssl
      ];
    in
    {
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        name = "lyricizer";
        version = "0.1.0";
        src = pkgs.lib.cleanSource ./.;

        useFetchCargoVendor = true;
        cargoHash = "sha256-VguFQMi2FEBtEAQj9Ak4UuWUXtjAQk46liAZ29zijFk=";
        nativeBuildInputs = buildInputs;
		inherit buildInputs;

        meta = {
          description = "downloads lyrics for songs in mpd";
          homepage = "https://github.com/CelestialCrafter/lyricizer";
          license = pkgs.lib.licenses.mpl20;
        };
      };

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages =
          with pkgs;
          [
            rustc
            cargo
          ]
          // buildInputs;
      };
    };
}
