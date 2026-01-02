{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      systems = lib.systems.flakeExposed;
      forAllSystems = lib.genAttrs systems;

      nixpkgsFor = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
        }
      );
    in
    {
      overlays.default = final: prev: {
        css-typing-gen = self.packages.${final.stdenv.system}.css-typing-gen;
      };

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        rec {
          css-typing-gen = default;
          default = pkgs.callPackage ./build.nix { };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              llvmPackages.lld

              openssl
              pkg-config
              gnumake
              pnpm_10
              wasm-bindgen-cli_0_2_106
              caddy # caddy file-server --listen :8000 --browse --root result
            ];
          };
        }
      );
    };
}
