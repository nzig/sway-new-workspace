{
  description = "Flake for github.com/nzig/sway-new-workspace";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    lib = pkgs.lib;
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "sway-new-workspace";
      version = "0.1.5";

      src = ./.;

      cargoHash = "sha256-9u3U4zpqStYpkayNV0H5ETyjQlwmSiw5QpkwjreteQE=";

      meta = with lib; {
        description = "A command to create new Sway workpaces";
        homepage = "https://github.com/nzig/sway-new-workspace";
        license = licenses.unlicense;
        maintainers = [maintainers.tailhook];
      };
    };
  };
}
