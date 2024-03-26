{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    src = pkgs.lib.cleanSource ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    meta = with pkgs.lib; {
      description = "Like Teamocil and iTermocil, but for Wezterm";
      homepage = "https://github.com/alexcaza/weztermocil";
      license = licenses.mit;
      maintainers = [];
    };
  }
