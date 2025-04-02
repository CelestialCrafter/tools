{
  inputs = {
  	nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    lyricizer = {
		url = "path:./lyricizer";
		inputs.nixpkgs.follows = "nixpkgs";
	};
    fetch = {
		url = "path:./fetch";
		inputs.nixpkgs.follows = "nixpkgs";
	};
  };

  outputs = { fetch, lyricizer, ... }: {
  	packages.x86_64-linux.fetch = fetch.packages.x86_64-linux.default;
  	packages.x86_64-linux.lyricizer = lyricizer.packages.x86_64-linux.default;
  };
}
