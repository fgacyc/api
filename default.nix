{ lib, stdenv, pgcli, sqlx-cli, dbmate, pkg-config, openssl }:

let
in stdenv.mkDerivation {
  pname = "api";
  version = "0.0.1";
  nativeBuildInputs = [ pgcli sqlx-cli dbmate pkg-config ];
  buildInputs = [ openssl ];
}
