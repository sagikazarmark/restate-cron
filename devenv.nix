{ pkgs, ... }:

{
  dotenv.enable = true;

  packages = with pkgs; [
    cargo-release
    cargo-watch
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
