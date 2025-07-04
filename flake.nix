{
  description = "Nix environment for development.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, ... }@inputs:
    inputs.utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        {
          devShells.default = pkgs.mkShell {
            name = "api";

            packages = with pkgs; [
              openssl
              pkg-config
              bun

              uv
              python312
              basedpyright
              ruff
              pgcli
              sqlx-cli
              dbmate
            ];

            shellHook = ''
              export SQLX_OFFLINE="true"
            '';
          };

          packages.default = pkgs.callPackage ./default.nix { };
        }
      );
}
