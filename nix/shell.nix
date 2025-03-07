{pkgs, ...}: let
  packages = with pkgs; [
    rust-bin.stable.latest.default

    cargo-watch
    sqlx-cli

    go

    protobuf
    go-protobuf
    protoc-gen-go-grpc

    goose
  ];

  libraries = with pkgs; [
    pkg-config
    openssl
  ];
in
  with pkgs;
    mkShell (let
      database_url = "postgres://postgres:postgres@localhost:5432";

      admin_database_url = "${database_url}/admin";
      client_database_url = "${database_url}/client";
      reservation_database_url = "${database_url}/reservation";
      coworking_database_url = "${database_url}/coworking";
    in {
      name = "backend";
      buildInputs = packages ++ libraries;

      DATABASE_URL = client_database_url;

      GOOSE_DRIVER = "postgres";
      GOOSE_DBSTRING = "postgres://postgres:postgres@localhost:5432/coworking?sslmode=disable";
      GOOSE_MIGRATION_DIR="./db/migrations";

      DIRENV_LOG_FORMAT = "";
      LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
    })
