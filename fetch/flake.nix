{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      packages.x86_64-linux.default = pkgs.buildGoModule {
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

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = [ pkgs.go ];
      };
    };
}
